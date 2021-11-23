/**
 * CLI process exit codes.
 *
 */
enum ExitCodes {
  OK = 0,

  InvalidParameters = 100,
  DevelopmentModeOnly,
  FileError,
  InvalidWorkerId,
  InvalidIntegerArray,
  ApiError = 200,
  UnsuccessfulRuntimeCall,
}

export = ExitCodes