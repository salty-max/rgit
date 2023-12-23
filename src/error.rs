pub type ExitCode = i32;

pub const SUCCESS: ExitCode = 0;
pub const ERROR_UNKNOWN: ExitCode = 1;
pub const ERROR_ARGUMENTS: ExitCode = 2;
pub const ERROR_ENVIRONMENT: ExitCode = 3;
pub const ERROR_FILESYSTEM: ExitCode = 4;
pub const ERROR_GIT: ExitCode = 5;
