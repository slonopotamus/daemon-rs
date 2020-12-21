pub mod daemon;
pub use crate::daemon::*;

mod singleton;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(unix)]
pub mod unix;
#[cfg(unix)]
pub use unix::*;
