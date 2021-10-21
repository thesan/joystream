import { ReadonlyConfig } from '../../types/config'
import { QueryNodeApi } from './query-node/api'
import { Logger } from 'winston'
import { LoggingService } from '../logging'
import { StorageNodeApi } from './storage-node/api'
import { PendingDownloadData, StateCacheService } from '../cache/StateCacheService'
import { DataObjectDetailsFragment } from './query-node/generated/queries'
import axios, { AxiosRequestConfig } from 'axios'
import {
  StorageNodeEndpointData,
  DataObjectAccessPoints,
  DataObjectData,
  DataObjectInfo,
  StorageNodeDownloadResponse,
  DownloadData,
} from '../../types'
import queue from 'queue'
import { DistributionBucketOperatorStatus } from './query-node/generated/schema'
import http from 'http'
import https from 'https'
import { parseAxiosError } from '../parsers/errors'

// Concurrency limits
export const MAX_CONCURRENT_AVAILABILITY_CHECKS_PER_DOWNLOAD = 10
export const MAX_CONCURRENT_RESPONSE_TIME_CHECKS = 10

export class NetworkingService {
  private config: ReadonlyConfig
  private queryNodeApi: QueryNodeApi
  // private runtimeApi: RuntimeApi
  private logging: LoggingService
  private stateCache: StateCacheService
  private logger: Logger

  private testLatencyQueue: queue
  private downloadQueue: queue

  constructor(config: ReadonlyConfig, stateCache: StateCacheService, logging: LoggingService) {
    axios.defaults.timeout = config.limits.outboundRequestsTimeout
    const httpConfig: http.AgentOptions | https.AgentOptions = {
      keepAlive: true,
      timeout: config.limits.outboundRequestsTimeout,
      maxSockets: config.limits.maxConcurrentOutboundConnections,
    }
    axios.defaults.httpAgent = new http.Agent(httpConfig)
    axios.defaults.httpsAgent = new https.Agent(httpConfig)
    this.config = config
    this.logging = logging
    this.stateCache = stateCache
    this.logger = logging.createLogger('NetworkingManager')
    this.queryNodeApi = new QueryNodeApi(config.endpoints.queryNode, this.logging)
    // this.runtimeApi = new RuntimeApi(config.endpoints.substrateNode)
    void this.checkActiveStorageNodeEndpoints()
    // Queues
    this.testLatencyQueue = queue({ concurrency: MAX_CONCURRENT_RESPONSE_TIME_CHECKS, autostart: true }).on(
      'end',
      () => {
        this.logger.verbose('Mean response times updated', {
          responseTimes: this.stateCache.getStorageNodeEndpointsMeanResponseTimes(),
        })
      }
    )
    this.downloadQueue = queue({ concurrency: config.limits.maxConcurrentStorageNodeDownloads, autostart: true })
  }

  private validateNodeEndpoint(endpoint: string): void {
    const endpointUrl = new URL(endpoint)
    if (endpointUrl.protocol !== 'http:' && endpointUrl.protocol !== 'https:') {
      throw new Error(`Invalid endpoint protocol: ${endpointUrl.protocol}`)
    }
  }

  private filterStorageNodeEndpoints(input: StorageNodeEndpointData[]): StorageNodeEndpointData[] {
    return input.filter((b) => {
      try {
        this.validateNodeEndpoint(b.endpoint)
        return true
      } catch (err) {
        this.logger.warn(`Invalid storage node endpoint: ${b.endpoint} for bucket ${b.bucketId}`, {
          bucketId: b.bucketId,
          endpoint: b.endpoint,
          err,
          '@pauseFor': 900,
        })
        return false
      }
    })
  }

  private getApiEndpoint(rootEndpoint: string) {
    return rootEndpoint.endsWith('/') ? rootEndpoint + 'api/v1' : rootEndpoint + '/api/v1'
  }

  private prepareStorageNodeEndpoints(details: DataObjectDetailsFragment) {
    const endpointsData = details.storageBag.storageAssignments
      .filter(
        (a) =>
          a.storageBucket.operatorStatus.__typename === 'StorageBucketOperatorStatusActive' &&
          a.storageBucket.operatorMetadata?.nodeEndpoint
      )
      .map((a) => {
        const rootEndpoint = a.storageBucket.operatorMetadata!.nodeEndpoint!
        const apiEndpoint = this.getApiEndpoint(rootEndpoint)
        return {
          bucketId: a.storageBucket.id,
          endpoint: apiEndpoint,
        }
      })

    return this.filterStorageNodeEndpoints(endpointsData)
  }

  private parseDataObjectAccessPoints(details: DataObjectDetailsFragment): DataObjectAccessPoints {
    return {
      storageNodes: this.prepareStorageNodeEndpoints(details),
    }
  }

  public async dataObjectInfo(objectId: string): Promise<DataObjectInfo> {
    const details = await this.queryNodeApi.getDataObjectDetails(objectId)
    return {
      exists: !!details,
      isSupported:
        (this.config.buckets === 'all' &&
          details?.storageBag.distirbutionAssignments.some((d) =>
            d.distributionBucket.operators.some(
              (o) => o.workerId === this.config.workerId && o.status === DistributionBucketOperatorStatus.Active
            )
          )) ||
        (Array.isArray(this.config.buckets) &&
          this.config.buckets.some((bucketId) =>
            details?.storageBag.distirbutionAssignments
              .map((a) => a.distributionBucket.id)
              .includes(bucketId.toString())
          )),
      data: details
        ? {
            objectId,
            accessPoints: this.parseDataObjectAccessPoints(details),
            contentHash: details.ipfsHash,
            size: parseInt(details.size),
          }
        : undefined,
    }
  }

  private sortEndpointsByMeanResponseTime(endpoints: string[]) {
    return endpoints.sort(
      (a, b) =>
        this.stateCache.getStorageNodeEndpointMeanResponseTime(a) -
        this.stateCache.getStorageNodeEndpointMeanResponseTime(b)
    )
  }

  private downloadJob(
    pendingDownload: PendingDownloadData,
    downloadData: DownloadData,
    onSourceFound: (response: StorageNodeDownloadResponse) => void,
    onError: (error: Error) => void,
    onFinished?: () => void
  ): Promise<void> {
    const {
      objectData: { objectId, accessPoints },
      startAt,
    } = downloadData

    pendingDownload.status = 'LookingForSource'

    return new Promise<void>((resolve, reject) => {
      // Handlers:
      const fail = (message: string) => {
        this.stateCache.dropPendingDownload(objectId)
        onError(new Error(message))
        reject(new Error(message))
      }

      const sourceFound = (response: StorageNodeDownloadResponse) => {
        this.logger.info('Download source chosen', { objectId, source: response.config.url })
        pendingDownload.status = 'Downloading'
        onSourceFound(response)
      }

      const finish = () => {
        onFinished && onFinished()
        resolve()
      }

      const storageEndpoints = this.sortEndpointsByMeanResponseTime(
        accessPoints?.storageNodes.map((n) => n.endpoint) || []
      )

      this.logger.info('Downloading new data object', {
        objectId,
        possibleSources: storageEndpoints.map((e) => ({
          endpoint: e,
          meanResponseTime: this.stateCache.getStorageNodeEndpointMeanResponseTime(e),
        })),
      })
      if (!storageEndpoints.length) {
        return fail('No storage endpoints available to download the data object from')
      }

      const availabilityQueue = queue({
        concurrency: MAX_CONCURRENT_AVAILABILITY_CHECKS_PER_DOWNLOAD,
        autostart: true,
      })
      const objectDownloadQueue = queue({ concurrency: 1, autostart: true })

      storageEndpoints.forEach(async (endpoint) => {
        availabilityQueue.push(async () => {
          const api = new StorageNodeApi(endpoint, this.logging)
          const available = await api.isObjectAvailable(objectId)
          if (!available) {
            throw new Error('Not avilable')
          }
          return endpoint
        })
      })

      availabilityQueue.on('success', (endpoint) => {
        availabilityQueue.stop()
        const job = async () => {
          const api = new StorageNodeApi(endpoint, this.logging)
          const response = await api.downloadObject(objectId, startAt)
          return response
        }
        objectDownloadQueue.push(job)
      })

      availabilityQueue.on('error', () => {
        /*
        Do nothing.
        The handler is needed to avoid unhandled promise rejection
        */
      })

      availabilityQueue.on('end', () => {
        if (!objectDownloadQueue.length) {
          fail('Failed to download the object from any availablable storage provider')
        }
      })

      objectDownloadQueue.on('error', (err) => {
        this.logger.error('Download attempt from storage node failed after availability was confirmed:', { err })
      })

      objectDownloadQueue.on('end', () => {
        if (availabilityQueue.length) {
          availabilityQueue.start()
        } else {
          fail('Failed to download the object from any availablable storage provider')
        }
      })

      objectDownloadQueue.on('success', (response: StorageNodeDownloadResponse) => {
        availabilityQueue.removeAllListeners().end()
        objectDownloadQueue.removeAllListeners().end()
        response.data.on('close', finish).on('error', finish).on('end', finish)
        sourceFound(response)
      })
    })
  }

  public downloadDataObject(downloadData: DownloadData): Promise<StorageNodeDownloadResponse> | null {
    const {
      objectData: { objectId, size },
    } = downloadData

    if (this.stateCache.getPendingDownload(objectId)) {
      // Already downloading
      return null
    }

    let resolveDownload: (response: StorageNodeDownloadResponse) => void, rejectDownload: (err: Error) => void
    const downloadPromise = new Promise<StorageNodeDownloadResponse>((resolve, reject) => {
      resolveDownload = resolve
      rejectDownload = reject
    })

    // Queue the download
    const pendingDownload = this.stateCache.newPendingDownload(objectId, size, downloadPromise)
    this.downloadQueue.push(() => this.downloadJob(pendingDownload, downloadData, resolveDownload, rejectDownload))

    return downloadPromise
  }

  async fetchSupportedDataObjects(): Promise<Map<string, DataObjectData>> {
    const data =
      this.config.buckets === 'all'
        ? await this.queryNodeApi.getDistributionBucketsWithObjectsByWorkerId(this.config.workerId)
        : await this.queryNodeApi.getDistributionBucketsWithObjectsByIds(this.config.buckets.map((id) => id.toString()))
    const objectsData = new Map<string, DataObjectData>()
    data.forEach((bucket) => {
      bucket.bagAssignments.forEach((a) => {
        a.storageBag.objects.forEach((object) => {
          const { ipfsHash, id, size } = object
          objectsData.set(id, { contentHash: ipfsHash, objectId: id, size: parseInt(size) })
        })
      })
    })

    return objectsData
  }

  async checkActiveStorageNodeEndpoints(): Promise<void> {
    try {
      const activeStorageOperators = await this.queryNodeApi.getActiveStorageBucketOperatorsData()
      const endpoints = this.filterStorageNodeEndpoints(
        activeStorageOperators.map(({ id, operatorMetadata }) => ({
          bucketId: id,
          endpoint: this.getApiEndpoint(operatorMetadata!.nodeEndpoint!),
        }))
      )
      this.logger.verbose('Checking nearby storage nodes...', { validEndpointsCount: endpoints.length })

      endpoints.forEach(({ endpoint }) =>
        this.testLatencyQueue.push(async () => {
          await this.checkResponseTime(endpoint)
        })
      )
    } catch (err) {
      this.logger.error("Couldn't check active storage node endpooints", { err })
    }
  }

  async checkResponseTime(endpoint: string): Promise<void> {
    const start = Date.now()
    this.logger.debug(`Sending storage node response-time check request to: ${endpoint}`, { endpoint })
    try {
      const api = new StorageNodeApi(endpoint, this.logging)
      const reqConfig: AxiosRequestConfig = { headers: { connection: 'close' } }
      await api.stateApi.stateApiGetVersion(reqConfig)
      const responseTime = Date.now() - start
      this.logger.debug(`${endpoint} check request response time: ${responseTime}`, { endpoint, responseTime })
      this.stateCache.setStorageNodeEndpointResponseTime(endpoint, responseTime)
    } catch (err) {
      if (axios.isAxiosError(err)) {
        const parsedErr = parseAxiosError(err)
        this.logger.warn(`${endpoint} check request error: ${parsedErr.message}`, {
          endpoint,
          err: parsedErr,
          '@pauseFor': 900,
        })
      } else {
        const message = err instanceof Error ? err.message : 'Unknown'
        this.logger.error(`${endpoint} check unexpected error: ${message}`, { endpoint, err, '@pauseFor': 900 })
      }
    }
  }
}