/* tslint:disable */
/* eslint-disable */
/**
 * Storage node API
 * Storage node API
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: info@joystream.org
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import { Configuration } from './configuration';
import globalAxios, { AxiosPromise, AxiosInstance, AxiosRequestConfig } from 'axios';
// Some imports not used depending on template conditions
// @ts-ignore
import { DUMMY_BASE_URL, assertParamExists, setApiKeyToObject, setBasicAuthToObject, setBearerAuthToObject, setOAuthToObject, setSearchParams, serializeDataIfNeeded, toPathString, createRequestFunction } from './common';
// @ts-ignore
import { BASE_PATH, COLLECTION_FORMATS, RequestArgs, BaseAPI, RequiredError } from './base';

/**
 * 
 * @export
 * @interface DataStatsResponse
 */
export interface DataStatsResponse {
    /**
     * 
     * @type {number}
     * @memberof DataStatsResponse
     */
    'totalSize': number;
    /**
     * 
     * @type {number}
     * @memberof DataStatsResponse
     */
    'objectNumber': number;
    /**
     * 
     * @type {number}
     * @memberof DataStatsResponse
     */
    'tempDirSize'?: number;
    /**
     * 
     * @type {number}
     * @memberof DataStatsResponse
     */
    'tempDownloads'?: number;
}
/**
 * 
 * @export
 * @interface ErrorResponse
 */
export interface ErrorResponse {
    /**
     * 
     * @type {string}
     * @memberof ErrorResponse
     */
    'type'?: string;
    /**
     * 
     * @type {string}
     * @memberof ErrorResponse
     */
    'message': string;
}
/**
 * 
 * @export
 * @interface FilesApiUploadFile201Response
 */
export interface FilesApiUploadFile201Response {
    /**
     * 
     * @type {string}
     * @memberof FilesApiUploadFile201Response
     */
    'id'?: string;
}
/**
 * 
 * @export
 * @interface StatusResponse
 */
export interface StatusResponse {
    /**
     * 
     * @type {string}
     * @memberof StatusResponse
     */
    'version': string;
    /**
     * 
     * @type {StatusResponseQueryNodeStatus}
     * @memberof StatusResponse
     */
    'queryNodeStatus': StatusResponseQueryNodeStatus;
}
/**
 * 
 * @export
 * @interface StatusResponseQueryNodeStatus
 */
export interface StatusResponseQueryNodeStatus {
    /**
     * 
     * @type {string}
     * @memberof StatusResponseQueryNodeStatus
     */
    'url': string;
    /**
     * 
     * @type {number}
     * @memberof StatusResponseQueryNodeStatus
     */
    'chainHead': number;
    /**
     * 
     * @type {number}
     * @memberof StatusResponseQueryNodeStatus
     */
    'blocksProcessed': number;
}
/**
 * 
 * @export
 * @interface TokenRequest
 */
export interface TokenRequest {
    /**
     * 
     * @type {TokenRequestData}
     * @memberof TokenRequest
     */
    'data': TokenRequestData;
    /**
     * 
     * @type {string}
     * @memberof TokenRequest
     */
    'signature': string;
}
/**
 * 
 * @export
 * @interface TokenRequestData
 */
export interface TokenRequestData {
    /**
     * 
     * @type {number}
     * @memberof TokenRequestData
     */
    'memberId': number;
    /**
     * 
     * @type {string}
     * @memberof TokenRequestData
     */
    'accountId': string;
    /**
     * 
     * @type {number}
     * @memberof TokenRequestData
     */
    'dataObjectId': number;
    /**
     * 
     * @type {number}
     * @memberof TokenRequestData
     */
    'storageBucketId': number;
    /**
     * 
     * @type {string}
     * @memberof TokenRequestData
     */
    'bagId': string;
}
/**
 * 
 * @export
 * @interface VersionResponse
 */
export interface VersionResponse {
    /**
     * 
     * @type {string}
     * @memberof VersionResponse
     */
    'version': string;
    /**
     * 
     * @type {string}
     * @memberof VersionResponse
     */
    'userAgent'?: string;
}

/**
 * FilesApi - axios parameter creator
 * @export
 */
export const FilesApiAxiosParamCreator = function (configuration?: Configuration) {
    return {
        /**
         * Returns a media file.
         * @param {string} id Data object ID
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        filesApiGetFile: async (id: string, options: AxiosRequestConfig = {}): Promise<RequestArgs> => {
            // verify required parameter 'id' is not null or undefined
            assertParamExists('filesApiGetFile', 'id', id)
            const localVarPath = `/files/{id}`
                .replace(`{${"id"}}`, encodeURIComponent(String(id)));
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, DUMMY_BASE_URL);
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }

            const localVarRequestOptions = { method: 'GET', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;


    
            setSearchParams(localVarUrlObj, localVarQueryParameter);
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};

            return {
                url: toPathString(localVarUrlObj),
                options: localVarRequestOptions,
            };
        },
        /**
         * Returns a media file headers.
         * @param {string} id Data object ID
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        filesApiGetFileHeaders: async (id: string, options: AxiosRequestConfig = {}): Promise<RequestArgs> => {
            // verify required parameter 'id' is not null or undefined
            assertParamExists('filesApiGetFileHeaders', 'id', id)
            const localVarPath = `/files/{id}`
                .replace(`{${"id"}}`, encodeURIComponent(String(id)));
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, DUMMY_BASE_URL);
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }

            const localVarRequestOptions = { method: 'HEAD', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;


    
            setSearchParams(localVarUrlObj, localVarQueryParameter);
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};

            return {
                url: toPathString(localVarUrlObj),
                options: localVarRequestOptions,
            };
        },
        /**
         * Upload data
         * @param {string} dataObjectId Data object runtime ID
         * @param {string} storageBucketId Storage bucket ID
         * @param {string} bagId Bag ID
         * @param {any} [file] Data file
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        filesApiUploadFile: async (dataObjectId: string, storageBucketId: string, bagId: string, file?: any, options: AxiosRequestConfig = {}): Promise<RequestArgs> => {
            // verify required parameter 'dataObjectId' is not null or undefined
            assertParamExists('filesApiUploadFile', 'dataObjectId', dataObjectId)
            // verify required parameter 'storageBucketId' is not null or undefined
            assertParamExists('filesApiUploadFile', 'storageBucketId', storageBucketId)
            // verify required parameter 'bagId' is not null or undefined
            assertParamExists('filesApiUploadFile', 'bagId', bagId)
            const localVarPath = `/files`;
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, DUMMY_BASE_URL);
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }

            const localVarRequestOptions = { method: 'POST', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;
            const localVarFormParams = new ((configuration && configuration.formDataCtor) || FormData)();

            if (dataObjectId !== undefined) {
                localVarQueryParameter['dataObjectId'] = dataObjectId;
            }

            if (storageBucketId !== undefined) {
                localVarQueryParameter['storageBucketId'] = storageBucketId;
            }

            if (bagId !== undefined) {
                localVarQueryParameter['bagId'] = bagId;
            }


            if (file !== undefined) { 
                localVarFormParams.append('file', file as any);
            }
    
    
            localVarHeaderParameter['Content-Type'] = 'multipart/form-data';
    
            setSearchParams(localVarUrlObj, localVarQueryParameter);
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};
            localVarRequestOptions.data = localVarFormParams;

            return {
                url: toPathString(localVarUrlObj),
                options: localVarRequestOptions,
            };
        },
    }
};

/**
 * FilesApi - functional programming interface
 * @export
 */
export const FilesApiFp = function(configuration?: Configuration) {
    const localVarAxiosParamCreator = FilesApiAxiosParamCreator(configuration)
    return {
        /**
         * Returns a media file.
         * @param {string} id Data object ID
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async filesApiGetFile(id: string, options?: AxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => AxiosPromise<any>> {
            const localVarAxiosArgs = await localVarAxiosParamCreator.filesApiGetFile(id, options);
            return createRequestFunction(localVarAxiosArgs, globalAxios, BASE_PATH, configuration);
        },
        /**
         * Returns a media file headers.
         * @param {string} id Data object ID
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async filesApiGetFileHeaders(id: string, options?: AxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => AxiosPromise<void>> {
            const localVarAxiosArgs = await localVarAxiosParamCreator.filesApiGetFileHeaders(id, options);
            return createRequestFunction(localVarAxiosArgs, globalAxios, BASE_PATH, configuration);
        },
        /**
         * Upload data
         * @param {string} dataObjectId Data object runtime ID
         * @param {string} storageBucketId Storage bucket ID
         * @param {string} bagId Bag ID
         * @param {any} [file] Data file
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async filesApiUploadFile(dataObjectId: string, storageBucketId: string, bagId: string, file?: any, options?: AxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => AxiosPromise<FilesApiUploadFile201Response>> {
            const localVarAxiosArgs = await localVarAxiosParamCreator.filesApiUploadFile(dataObjectId, storageBucketId, bagId, file, options);
            return createRequestFunction(localVarAxiosArgs, globalAxios, BASE_PATH, configuration);
        },
    }
};

/**
 * FilesApi - factory interface
 * @export
 */
export const FilesApiFactory = function (configuration?: Configuration, basePath?: string, axios?: AxiosInstance) {
    const localVarFp = FilesApiFp(configuration)
    return {
        /**
         * Returns a media file.
         * @param {string} id Data object ID
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        filesApiGetFile(id: string, options?: any): AxiosPromise<any> {
            return localVarFp.filesApiGetFile(id, options).then((request) => request(axios, basePath));
        },
        /**
         * Returns a media file headers.
         * @param {string} id Data object ID
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        filesApiGetFileHeaders(id: string, options?: any): AxiosPromise<void> {
            return localVarFp.filesApiGetFileHeaders(id, options).then((request) => request(axios, basePath));
        },
        /**
         * Upload data
         * @param {string} dataObjectId Data object runtime ID
         * @param {string} storageBucketId Storage bucket ID
         * @param {string} bagId Bag ID
         * @param {any} [file] Data file
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        filesApiUploadFile(dataObjectId: string, storageBucketId: string, bagId: string, file?: any, options?: any): AxiosPromise<FilesApiUploadFile201Response> {
            return localVarFp.filesApiUploadFile(dataObjectId, storageBucketId, bagId, file, options).then((request) => request(axios, basePath));
        },
    };
};

/**
 * FilesApi - object-oriented interface
 * @export
 * @class FilesApi
 * @extends {BaseAPI}
 */
export class FilesApi extends BaseAPI {
    /**
     * Returns a media file.
     * @param {string} id Data object ID
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof FilesApi
     */
    public filesApiGetFile(id: string, options?: AxiosRequestConfig) {
        return FilesApiFp(this.configuration).filesApiGetFile(id, options).then((request) => request(this.axios, this.basePath));
    }

    /**
     * Returns a media file headers.
     * @param {string} id Data object ID
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof FilesApi
     */
    public filesApiGetFileHeaders(id: string, options?: AxiosRequestConfig) {
        return FilesApiFp(this.configuration).filesApiGetFileHeaders(id, options).then((request) => request(this.axios, this.basePath));
    }

    /**
     * Upload data
     * @param {string} dataObjectId Data object runtime ID
     * @param {string} storageBucketId Storage bucket ID
     * @param {string} bagId Bag ID
     * @param {any} [file] Data file
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof FilesApi
     */
    public filesApiUploadFile(dataObjectId: string, storageBucketId: string, bagId: string, file?: any, options?: AxiosRequestConfig) {
        return FilesApiFp(this.configuration).filesApiUploadFile(dataObjectId, storageBucketId, bagId, file, options).then((request) => request(this.axios, this.basePath));
    }
}


/**
 * StateApi - axios parameter creator
 * @export
 */
export const StateApiAxiosParamCreator = function (configuration?: Configuration) {
    return {
        /**
         * Returns all local data objects.
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        stateApiGetAllLocalDataObjects: async (options: AxiosRequestConfig = {}): Promise<RequestArgs> => {
            const localVarPath = `/state/data-objects`;
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, DUMMY_BASE_URL);
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }

            const localVarRequestOptions = { method: 'GET', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;


    
            setSearchParams(localVarUrlObj, localVarQueryParameter);
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};

            return {
                url: toPathString(localVarUrlObj),
                options: localVarRequestOptions,
            };
        },
        /**
         * Returns local data objects for the bag.
         * @param {string} bagId Bag ID
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        stateApiGetLocalDataObjectsByBagId: async (bagId: string, options: AxiosRequestConfig = {}): Promise<RequestArgs> => {
            // verify required parameter 'bagId' is not null or undefined
            assertParamExists('stateApiGetLocalDataObjectsByBagId', 'bagId', bagId)
            const localVarPath = `/state/bags/{bagId}/data-objects`
                .replace(`{${"bagId"}}`, encodeURIComponent(String(bagId)));
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, DUMMY_BASE_URL);
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }

            const localVarRequestOptions = { method: 'GET', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;


    
            setSearchParams(localVarUrlObj, localVarQueryParameter);
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};

            return {
                url: toPathString(localVarUrlObj),
                options: localVarRequestOptions,
            };
        },
        /**
         * Returns local uploading directory stats.
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        stateApiGetLocalDataStats: async (options: AxiosRequestConfig = {}): Promise<RequestArgs> => {
            const localVarPath = `/state/data`;
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, DUMMY_BASE_URL);
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }

            const localVarRequestOptions = { method: 'GET', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;


    
            setSearchParams(localVarUrlObj, localVarQueryParameter);
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};

            return {
                url: toPathString(localVarUrlObj),
                options: localVarRequestOptions,
            };
        },
        /**
         * Returns json object describing current node status.
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        stateApiGetStatus: async (options: AxiosRequestConfig = {}): Promise<RequestArgs> => {
            const localVarPath = `/status`;
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, DUMMY_BASE_URL);
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }

            const localVarRequestOptions = { method: 'GET', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;


    
            setSearchParams(localVarUrlObj, localVarQueryParameter);
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};

            return {
                url: toPathString(localVarUrlObj),
                options: localVarRequestOptions,
            };
        },
        /**
         * Returns server version.
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        stateApiGetVersion: async (options: AxiosRequestConfig = {}): Promise<RequestArgs> => {
            const localVarPath = `/version`;
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, DUMMY_BASE_URL);
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }

            const localVarRequestOptions = { method: 'GET', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;


    
            setSearchParams(localVarUrlObj, localVarQueryParameter);
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};

            return {
                url: toPathString(localVarUrlObj),
                options: localVarRequestOptions,
            };
        },
    }
};

/**
 * StateApi - functional programming interface
 * @export
 */
export const StateApiFp = function(configuration?: Configuration) {
    const localVarAxiosParamCreator = StateApiAxiosParamCreator(configuration)
    return {
        /**
         * Returns all local data objects.
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async stateApiGetAllLocalDataObjects(options?: AxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => AxiosPromise<Array<string>>> {
            const localVarAxiosArgs = await localVarAxiosParamCreator.stateApiGetAllLocalDataObjects(options);
            return createRequestFunction(localVarAxiosArgs, globalAxios, BASE_PATH, configuration);
        },
        /**
         * Returns local data objects for the bag.
         * @param {string} bagId Bag ID
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async stateApiGetLocalDataObjectsByBagId(bagId: string, options?: AxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => AxiosPromise<Array<string>>> {
            const localVarAxiosArgs = await localVarAxiosParamCreator.stateApiGetLocalDataObjectsByBagId(bagId, options);
            return createRequestFunction(localVarAxiosArgs, globalAxios, BASE_PATH, configuration);
        },
        /**
         * Returns local uploading directory stats.
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async stateApiGetLocalDataStats(options?: AxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => AxiosPromise<DataStatsResponse>> {
            const localVarAxiosArgs = await localVarAxiosParamCreator.stateApiGetLocalDataStats(options);
            return createRequestFunction(localVarAxiosArgs, globalAxios, BASE_PATH, configuration);
        },
        /**
         * Returns json object describing current node status.
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async stateApiGetStatus(options?: AxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => AxiosPromise<StatusResponse>> {
            const localVarAxiosArgs = await localVarAxiosParamCreator.stateApiGetStatus(options);
            return createRequestFunction(localVarAxiosArgs, globalAxios, BASE_PATH, configuration);
        },
        /**
         * Returns server version.
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async stateApiGetVersion(options?: AxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => AxiosPromise<VersionResponse>> {
            const localVarAxiosArgs = await localVarAxiosParamCreator.stateApiGetVersion(options);
            return createRequestFunction(localVarAxiosArgs, globalAxios, BASE_PATH, configuration);
        },
    }
};

/**
 * StateApi - factory interface
 * @export
 */
export const StateApiFactory = function (configuration?: Configuration, basePath?: string, axios?: AxiosInstance) {
    const localVarFp = StateApiFp(configuration)
    return {
        /**
         * Returns all local data objects.
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        stateApiGetAllLocalDataObjects(options?: any): AxiosPromise<Array<string>> {
            return localVarFp.stateApiGetAllLocalDataObjects(options).then((request) => request(axios, basePath));
        },
        /**
         * Returns local data objects for the bag.
         * @param {string} bagId Bag ID
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        stateApiGetLocalDataObjectsByBagId(bagId: string, options?: any): AxiosPromise<Array<string>> {
            return localVarFp.stateApiGetLocalDataObjectsByBagId(bagId, options).then((request) => request(axios, basePath));
        },
        /**
         * Returns local uploading directory stats.
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        stateApiGetLocalDataStats(options?: any): AxiosPromise<DataStatsResponse> {
            return localVarFp.stateApiGetLocalDataStats(options).then((request) => request(axios, basePath));
        },
        /**
         * Returns json object describing current node status.
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        stateApiGetStatus(options?: any): AxiosPromise<StatusResponse> {
            return localVarFp.stateApiGetStatus(options).then((request) => request(axios, basePath));
        },
        /**
         * Returns server version.
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        stateApiGetVersion(options?: any): AxiosPromise<VersionResponse> {
            return localVarFp.stateApiGetVersion(options).then((request) => request(axios, basePath));
        },
    };
};

/**
 * StateApi - object-oriented interface
 * @export
 * @class StateApi
 * @extends {BaseAPI}
 */
export class StateApi extends BaseAPI {
    /**
     * Returns all local data objects.
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof StateApi
     */
    public stateApiGetAllLocalDataObjects(options?: AxiosRequestConfig) {
        return StateApiFp(this.configuration).stateApiGetAllLocalDataObjects(options).then((request) => request(this.axios, this.basePath));
    }

    /**
     * Returns local data objects for the bag.
     * @param {string} bagId Bag ID
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof StateApi
     */
    public stateApiGetLocalDataObjectsByBagId(bagId: string, options?: AxiosRequestConfig) {
        return StateApiFp(this.configuration).stateApiGetLocalDataObjectsByBagId(bagId, options).then((request) => request(this.axios, this.basePath));
    }

    /**
     * Returns local uploading directory stats.
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof StateApi
     */
    public stateApiGetLocalDataStats(options?: AxiosRequestConfig) {
        return StateApiFp(this.configuration).stateApiGetLocalDataStats(options).then((request) => request(this.axios, this.basePath));
    }

    /**
     * Returns json object describing current node status.
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof StateApi
     */
    public stateApiGetStatus(options?: AxiosRequestConfig) {
        return StateApiFp(this.configuration).stateApiGetStatus(options).then((request) => request(this.axios, this.basePath));
    }

    /**
     * Returns server version.
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof StateApi
     */
    public stateApiGetVersion(options?: AxiosRequestConfig) {
        return StateApiFp(this.configuration).stateApiGetVersion(options).then((request) => request(this.axios, this.basePath));
    }
}

