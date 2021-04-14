//use std::cmp;
use std::io;
use std::os::unix::io::AsRawFd;
use std::vec::Vec;
use sgx_libc as libc;
use iovec::IoVec;
//use iovec::unix as iovec;

pub trait VecIo {
    fn readv(&self, bufs: &mut [&mut IoVec]) -> io::Result<usize>;

    fn writev(&self, bufs: &[&IoVec]) -> io::Result<usize>;
}

impl<T: AsRawFd> VecIo for T {
    fn readv(&self, bufs: &mut [&mut IoVec]) -> io::Result<usize> {
        let vbufs: Vec<&mut [u8]> = bufs.iter_mut().map(|msl| msl.as_mut_bytes()).collect();
        unsafe {
            ::cvt_ocall(libc::ocall::readv(self.as_raw_fd(), vbufs))
        }
    }

    fn writev(&self, bufs: &[&IoVec]) -> io::Result<usize> {
        unsafe {
            let vbufs: Vec<&[u8]> = bufs.iter().map(|msl| msl.as_bytes()).collect();
            ::cvt_ocall(libc::ocall::writev(self.as_raw_fd(), vbufs))
        }
    }
}
