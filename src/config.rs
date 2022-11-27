use build_time::build_time_local;
use git_version::git_version;

/// Date (Y-m-d) of when the program was compiled.
pub const BUILD_DATE: &str = build_time_local!("%Y-%m-%d");

/// Git version of the build.
pub const GIT_VERSION: &str = git_version!();

#[cfg(debug_assertions)]
pub const DEFAULT_VOLUME: f32 = 0.0;

#[cfg(not(debug_assertions))]
pub const DEFAULT_VOLUME: f32 = 1.0;