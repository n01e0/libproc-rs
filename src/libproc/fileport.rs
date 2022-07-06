extern crate libc;

use crate::osx_libproc_bindings::{
    self, proc_pidfileportinfo, 
};
use libc::{c_void};

use std::mem;
use super::helpers;

/// pidfileportinfo flavor
pub trait PIDFilePortInfo {
    /// Return the `PIDFilePortInfoFlavor` of the implementing struct
    fn flavor() -> PidFilePortInfoFlavor;
}

/// An enum used to specify what type of information about a fileport
pub enum PidFilePortInfoFlavor {
    /// for get vnode_fdinfowithpath
    Vnode = osx_libproc_bindings::PROC_PIDFILEPORTVNODEPATHINFO as isize,
    /// for get pipe_fdinfo
    Pipe = osx_libproc_bindings::PROC_PIDFILEPORTPIPEINFO as isize,
    /// for get pshm_fdinfo
    Shm = osx_libproc_bindings::PROC_PIDFILEPORTPSHMINFO as isize,
    /// for get socket_fdinfo
    Socket = osx_libproc_bindings::PROC_PIDFILEPORTSOCKETINFO as isize,
}

impl PIDFilePortInfo for osx_libproc_bindings::vnode_fdinfowithpath {
    fn flavor() -> PidFilePortInfoFlavor { PidFilePortInfoFlavor::Vnode }
}

impl PIDFilePortInfo for osx_libproc_bindings::pipe_fdinfo {
    fn flavor() -> PidFilePortInfoFlavor { PidFilePortInfoFlavor::Pipe }
}

impl PIDFilePortInfo for osx_libproc_bindings::pshm_fdinfo {
    fn flavor() -> PidFilePortInfoFlavor { PidFilePortInfoFlavor::Shm }
}

impl PIDFilePortInfo for osx_libproc_bindings::socket_fdinfo {
    fn flavor() -> PidFilePortInfoFlavor { PidFilePortInfoFlavor::Socket }
}

/// Get fileport info
pub fn pidfileportinfo<T: PIDFilePortInfo>(pid: i32, fileport: u32) -> Result<T, String> {
    let flavor = T::flavor() as i32;
    let buffer_size = mem::size_of::<T>() as i32;
    let mut fileportinfo = unsafe { mem::zeroed() };
    let buffer_ptr = &mut fileportinfo as *mut _ as *mut c_void;
    let ret: i32;

    unsafe {
        ret = proc_pidfileportinfo(pid, fileport, flavor, buffer_ptr, buffer_size);
    }

    if ret <= 0 {
        Err(helpers::get_errno_with_message(ret))
    } else {
        Ok(fileportinfo)
    }
}
