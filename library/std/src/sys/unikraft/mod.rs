pub mod alloc;
#[path = "../unsupported/args.rs"]
pub mod args;
#[path = "../unix/cmath.rs"]
pub mod cmath;
#[path = "../unsupported/env.rs"]
pub mod env;
#[path = "../unsupported/fs.rs"]
pub mod fs;
#[path = "../unsupported/io.rs"]
pub mod io;
#[path = "../unsupported/locks/mod.rs"]
pub mod locks;
#[path = "../unsupported/net.rs"]
pub mod net;
#[path = "../unsupported/once.rs"]
pub mod once;
#[path = "../unsupported/os.rs"]
pub mod os;
#[path = "../unix/os_str.rs"]
pub mod os_str;
#[path = "../unix/path.rs"]
pub mod path;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
pub mod stdio;
#[path = "../unsupported/thread.rs"]
pub mod thread;
#[path = "../unsupported/thread_local_dtor.rs"]
pub mod thread_local_dtor;
#[path = "../unsupported/thread_local_key.rs"]
pub mod thread_local_key;
#[path = "../unsupported/time.rs"]
pub mod time;

use crate::io as std_io;

fn cvt<T>(res: unikraft_abi::io::Result<T>) -> crate::io::Result<T> {
    res.map_err(i32::from).map_err(crate::io::Error::from_raw_os_error)
}

pub mod memchr {
    pub use core::slice::memchr::{memchr, memrchr};
}

// SAFETY: must be called only once during runtime initialization.
// NOTE: this is not guaranteed to run, for example when Rust code is called externally.
pub unsafe fn init(_argc: isize, _argv: *const *const u8, _sigpipe: u8) {}

// SAFETY: must be called only once during runtime cleanup.
// NOTE: this is not guaranteed to run, for example when the program aborts.
pub unsafe fn cleanup() {}

pub fn unsupported<T>() -> std_io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> std_io::Error {
    std_io::const_io_error!(
        std_io::ErrorKind::Unsupported,
        "operation not supported on this platform",
    )
}

pub fn decode_error_kind(_code: i32) -> crate::io::ErrorKind {
    crate::io::ErrorKind::Uncategorized
}

pub fn abort_internal() -> ! {
    use unikraft_abi::guest::{self, ExitMode};

    guest::exit(ExitMode::Crash)
}

pub fn hashmap_random_keys() -> (u64, u64) {
    (1, 2)
}
