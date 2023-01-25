//! Unikraft-specific definitions.

#![stable(feature = "raw_ext", since = "1.1.0")]
#![doc(cfg(target_os = "unikraft"))]

pub mod fs;
pub mod raw;
