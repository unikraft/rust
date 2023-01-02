use unikraft_abi::io as uk_io;

use crate::io;

pub struct Stdin(uk_io::Stdin);

impl Stdin {
    pub const fn new() -> Self {
        Self(uk_io::Stdin::new())
    }
}

impl io::Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        super::cvt(uk_io::Read::read(&mut self.0, buf))
    }
}

pub struct Stdout(uk_io::Stdout);

impl Stdout {
    pub const fn new() -> Self {
        Self(uk_io::Stdout::new())
    }
}

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        super::cvt(uk_io::Write::write(&mut self.0, buf))
    }

    fn flush(&mut self) -> io::Result<()> {
        super::cvt(uk_io::Write::flush(&mut self.0))
    }
}

pub struct Stderr(uk_io::Stderr);

impl Stderr {
    pub const fn new() -> Self {
        Self(uk_io::Stderr::new())
    }
}

impl io::Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        super::cvt(uk_io::Write::write(&mut self.0, buf))
    }

    fn flush(&mut self) -> io::Result<()> {
        super::cvt(uk_io::Write::flush(&mut self.0))
    }
}

pub const STDIN_BUF_SIZE: usize = crate::sys_common::io::DEFAULT_BUF_SIZE;

pub fn is_ebadf(err: &io::Error) -> bool {
    matches!(err.raw_os_error(), Some(errno) if errno == uk_io::errno::EBADF)
}

pub fn panic_output() -> Option<impl io::Write> {
    Some(Stderr::new())
}
