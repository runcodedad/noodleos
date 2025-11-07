/// Architecture-specific code
/// 
/// This module provides architecture-specific implementations.
/// Currently supports x86_64, but can be extended for other architectures.

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

// Re-export the current architecture's functionality
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
