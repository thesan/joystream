import { SubmittableResult } from '@polkadot/api'
import { KeyringPair } from '@polkadot/keyring/types'
import { RuntimeApi } from '../RuntimeApi'
import { Keyring } from '@polkadot/keyring'
import { Logger } from 'winston'
import path from 'path'
import nodeCleanup from 'node-cleanup'
import _ from 'lodash'
import fs from 'fs'
import { SubmittableExtrinsic } from '@polkadot/api/types'

export type MigrationResult = {
  idsMap: Map<number, number>
  failedMigrations: number[]
}

export type MigrationStateJson = {
  idsMapEntries: [number, number][]
  failedMigrations: number[]
}

export type BaseMigrationConfig = {
  migrationStatePath: string
  sudoUri: string
}

export type BaseMigrationParams<T> = {
  api: RuntimeApi
  snapshot: T
  config: BaseMigrationConfig
}

export abstract class BaseMigration<T> {
  abstract readonly name: string
  protected api: RuntimeApi
  protected sudo!: KeyringPair
  protected config: BaseMigrationConfig
  protected snapshot: T
  protected failedMigrations: Set<number>
  protected idsMap: Map<number, number>
  protected pendingMigrationIteration: Promise<void> | undefined
  protected abstract logger: Logger

  public constructor({ api, config, snapshot }: BaseMigrationParams<T>) {
    this.api = api
    this.config = config
    this.failedMigrations = new Set()
    this.idsMap = new Map()
    this.snapshot = snapshot
    fs.mkdirSync(config.migrationStatePath, { recursive: true })
  }

  protected getMigrationStateFilePath(): string {
    const { migrationStatePath } = this.config
    return path.join(migrationStatePath, `${_.camelCase(this.name)}.json`)
  }

  public async init(): Promise<void> {
    this.loadMigrationState()
    nodeCleanup(this.onExit.bind(this))
    await this.loadSudoKey()
  }

  public abstract run(): Promise<MigrationResult>

  protected abstract migrateBatch(batchTx: SubmittableExtrinsic<'promise'>, batch: { id: string }[]): Promise<void>

  protected getMigrationStateJson(): MigrationStateJson {
    return {
      idsMapEntries: Array.from(this.idsMap.entries()),
      failedMigrations: Array.from(this.failedMigrations),
    }
  }

  protected loadMigrationState(): void {
    const stateFilePath = this.getMigrationStateFilePath()
    if (fs.existsSync(stateFilePath)) {
      const migrationStateJson = fs.readFileSync(stateFilePath).toString()
      const migrationState: MigrationStateJson = JSON.parse(migrationStateJson)
      this.idsMap = new Map(migrationState.idsMapEntries)
    }
  }

  protected onExit(exitCode: number | null, signal: string | null): void | false {
    nodeCleanup.uninstall() // don't call cleanup handler again
    this.logger.info('Exitting...')
    if (signal && this.pendingMigrationIteration !== undefined) {
      this.logger.info('Waiting for currently pending migration iteration to finalize...')
      this.pendingMigrationIteration.then(() => {
        this.saveMigrationState(true)
        this.logger.info('Done.')
        process.kill(process.pid, signal)
      })
      return false
    } else {
      this.saveMigrationState(true)
      this.logger.info('Done.')
    }
  }

  protected saveMigrationState(isExitting: boolean): void {
    this.logger.info(`Saving ${isExitting ? 'final' : 'intermediate'} migration state...`)
    const stateFilePath = this.getMigrationStateFilePath()
    const migrationState = this.getMigrationStateJson()
    fs.writeFileSync(stateFilePath, JSON.stringify(migrationState, undefined, 2))
  }

  private async loadSudoKey() {
    const { sudoUri } = this.config
    const keyring = new Keyring({ type: 'sr25519', ss58Format: 126 })
    this.sudo = keyring.createFromUri(sudoUri)
    const sudoKey = await this.api.query.sudo.key()
    if (sudoKey.toString() !== this.sudo.address) {
      throw new Error(`Invalid sudo key! Expected: ${sudoKey.toString()}, Got: ${this.sudo.address}`)
    }
  }

  protected async executeBatchMigration<T extends { id: string }>(
    batchTx: SubmittableExtrinsic<'promise'>,
    batch: T[]
  ): Promise<void> {
    this.pendingMigrationIteration = (async () => {
      await this.migrateBatch(batchTx, batch)
      this.saveMigrationState(false)
    })()
    await this.pendingMigrationIteration
    this.pendingMigrationIteration = undefined
  }

  /**
   * Extract failed migrations (entity ids) from batch transaction result.
   * Assumptions:
   * - Each entity is migrated with a constant number of calls (1 by default: sudo.sudo)
   * - Ordering of the entities in the `batch` array matches the ordering of the batched calls through which they are migrated
   *
   * Entity migration is considered failed if the last call (per entity) failed or was not executed at all, regardless of
   * the result of any of the previous calls associated with that entity migration.
   */
  protected extractFailedMigrations<T extends { id: string }>(
    result: SubmittableResult,
    batch: T[],
    callsPerEntity = 1,
    usesSudo = true
  ): void {
    const { api } = this
    const batchInterruptedEvent = api.findEvent(result, 'utility', 'BatchInterrupted')
    const numberOfSuccesfulCalls = batchInterruptedEvent
      ? batchInterruptedEvent.data[0].toNumber()
      : callsPerEntity * batch.length
    const numberOfMigratedEntites = Math.floor(numberOfSuccesfulCalls / callsPerEntity)

    const sudoSudidEvents = api.findEvents(result, 'sudo', 'Sudid')

    if (usesSudo && sudoSudidEvents.length !== numberOfMigratedEntites) {
      throw new Error(
        `Unexpected number of Sudid events (expected: ${numberOfMigratedEntites}, got: ${sudoSudidEvents.length})! ` +
          `Could not extract failed migrations from: ${JSON.stringify(result.toHuman())}`
      )
    }

    const failedIds: number[] = []
    batch.forEach((entity, i) => {
      const entityId = parseInt(entity.id)
      if (i >= numberOfMigratedEntites || (usesSudo && sudoSudidEvents[i].data[0].isErr)) {
        failedIds.push(entityId)
        this.failedMigrations.add(entityId)
      }
    })

    if (batchInterruptedEvent) {
      const spRuntimeError = batchInterruptedEvent.data[1]
      this.logger.error(
        `Batch interrupted at call ${numberOfSuccesfulCalls}: ${JSON.stringify(spRuntimeError.toHuman())}`
      )
    }

    if (failedIds.length) {
      this.logger.error(`Failed to migrate:`, { failedIds })
    }
  }

  public getResult(): MigrationResult {
    const { idsMap, failedMigrations } = this
    return {
      idsMap: new Map(idsMap.entries()),
      failedMigrations: Array.from(failedMigrations),
    }
  }
}