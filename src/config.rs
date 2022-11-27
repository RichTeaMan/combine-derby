use build_time::build_time_local;
use git_version::git_version;

/// Date (Y-m-d) of when the program was compiled.
pub const BUILD_DATE: &str = build_time_local!("%Y-%m-%d");

/// Git version of the build.
pub const GIT_VERSION: &str = git_version!();

pub const SOUND_ENABLED: bool = false;
