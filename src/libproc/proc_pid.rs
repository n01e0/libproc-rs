extern crate libc;

use std::env;
#[cfg(target_os = "macos")]
use crate::errno::errno;
use super::error::LibProcError;
use anyhow::{Context, Result};
#[cfg(target_os = "linux")]
use std::ffi::CString;
#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "macos")]
use std::mem;
use std::path::PathBuf;
#[cfg(target_os = "macos")]
use std::ptr;

use libc::pid_t;
#[cfg(target_os = "linux")]
use libc::PATH_MAX;

#[cfg(target_os = "macos")]
use crate::libproc::bsd_info::BSDInfo;
use crate::libproc::helpers;
#[cfg(target_os = "macos")]
use crate::libproc::task_info::{TaskAllInfo, TaskInfo};
#[cfg(target_os = "macos")]
use crate::libproc::thread_info::ThreadInfo;
use crate::libproc::work_queue_info::WorkQueueInfo;
#[cfg(target_os = "macos")]
use crate::osx_libproc_bindings::{
    proc_libversion, proc_listpids, proc_listpidspath, proc_name, proc_pidinfo, proc_pidpath,
    proc_regionfilename, PROC_PIDPATHINFO_MAXSIZE,
};

#[cfg(target_os = "macos")]
use self::libc::{c_char, c_void};
#[cfg(target_os = "linux")]
use self::libc::{c_char, readlink};

#[cfg(target_os = "macos")]
use std::ffi::CString;

/// The `ProcType` type. Used to specify what type of processes you are interested
/// in in other calls, such as `listpids`.
#[derive(Copy, Clone)]
pub enum ProcType {
    /// All processes
    ProcAllPIDS = 1,
    /// Only PGRP Processes
    ProcPGRPOnly = 2,
    /// Only TTY Processes
    ProcTTYOnly = 3,
    /// Only UID Processes
    ProcUIDOnly = 4,
    /// Only RUID Processes
    ProcRUIDOnly = 5,
    /// Only PPID Processes
    ProcPPIDOnly = 6,
}

/// The `PIDInfo` trait is needed for polymorphism on pidinfo types, also abstracting flavor in order to provide
/// type-guaranteed flavor correctness
pub trait PIDInfo {
    /// Return the `PidInfoFlavor` of the implementing struct
    fn flavor() -> PidInfoFlavor;
}

/// An enum used to specify what type of information about a process is referenced
/// See <http://opensource.apple.com/source/xnu/xnu-1504.7.4/bsd/kern/proc_info.c>
pub enum PidInfoFlavor {
    /// List of File Descriptors
    ListFDs = 1,
    /// struct proc_taskallinfo
    TaskAllInfo = 2,
    /// struct proc_bsdinfo
    TBSDInfo = 3,
    /// struct proc_taskinfo
    TaskInfo = 4,
    /// struct proc_threadinfo
    ThreadInfo = 5,
    /// list if int thread ids
    ListThreads = 6,
    /// TBD what type RegionInfo is - string?
    RegionInfo = 7,
    /// Region Path info strings
    RegionPathInfo = 8,
    /// Strings
    VNodePathInfo = 9,
    /// Strings
    ThreadPathInfo = 10,
    /// Strings
    PathInfo = 11,
    /// struct proc_workqueueinfo
    WorkQueueInfo = 12,
}

/// The `PidInfo` enum contains a piece of information about a processes
#[allow(clippy::large_enum_variant)]
pub enum PidInfo {
    /// File Descriptors used by Process
    ListFDs(Vec<i32>),
    /// Get all Task Info
    #[cfg(target_os = "macos")]
    TaskAllInfo(TaskAllInfo),
    /// Get TBSDInfo - TODO doc this
    #[cfg(target_os = "macos")]
    TBSDInfo(BSDInfo),
    /// Single Task Info
    #[cfg(target_os = "macos")]
    TaskInfo(TaskInfo),
    /// ThreadInfo
    #[cfg(target_os = "macos")]
    ThreadInfo(ThreadInfo),
    /// A list of Thread IDs
    ListThreads(Vec<i32>),
    /// RegionInfo
    RegionInfo(String),
    /// RegionPathInfo
    RegionPathInfo(String),
    /// VNodePathInfo
    VNodePathInfo(String),
    /// ThreadPathInfo
    ThreadPathInfo(String),
    /// The path of the executable being run as the process
    PathInfo(String),
    /// WorkQueueInfo
    WorkQueueInfo(WorkQueueInfo),
    /// A list of Fileport info
    #[cfg(target_os = "macos")]
    ListFilePorts(Vec<FilePortInfo>),
}

/// The `FDType`
pub enum FDType {
    /// vnode
    VNode = 1,
    /// socket
    Sockt = 2,
    /// pshm
    PSHM = 3,
    /// psem
    PSEM = 4,
    /// kqueue
    KQueue = 5,
    /// pipe
    Pipe = 6,
    /// fsevents
    FSEvents = 7,
    /// netpolicy
    NetPolicy = 9,
    /// channel
    Channel = 10,
    /// nexus
    Nexus = 11,
}

/// The `ListPIDInfo` trait is needed for polymorphism on listpidinfo types, also abstracting flavor in order to provide
/// type-guaranteed flavor correctness
pub trait ListPIDInfo {
    /// Item
    type Item;
    /// Return the PidInfoFlavor of the implementing struct
    fn flavor() -> PidInfoFlavor;
}

/// Struct for List of Threads
pub struct ListThreads;

impl ListPIDInfo for ListThreads {
    type Item = u64;
    fn flavor() -> PidInfoFlavor {
        PidInfoFlavor::ListThreads
    }
}

/// Struct fileportinfo
#[repr(C)]
pub struct FilePortInfo {
    /// fileport
    pub proc_fileport: u32,
    /// fdtype
    pub proc_fdtype: u32,
}

/// Returns the PIDs of the active processes that match the ProcType passed in
///
/// # Examples
///
/// Get the list of running process IDs using `listpids`
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid;
///
/// if let Ok(pids) = proc_pid::listpids(proc_pid::ProcType::ProcAllPIDS) {
///     println!("Found {} processes using listpids()", pids.len());
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn listpids(proc_types: ProcType) -> Result<Vec<u32>> {
    let buffer_size = unsafe { proc_listpids(proc_types as u32, 0, ptr::null_mut(), 0) };
    if buffer_size <= 0 {
        return Err(LibProcError::from(errno()).into());
    }

    let capacity = buffer_size as usize / mem::size_of::<u32>();
    let mut pids: Vec<u32> = Vec::with_capacity(capacity);
    let buffer_ptr = pids.as_mut_ptr() as *mut c_void;

    let ret = unsafe { proc_listpids(proc_types as u32, 0, buffer_ptr, buffer_size as i32) };

    if ret <= 0 {
        Err(errno().into())
    } else {
        let items_count = ret as usize / mem::size_of::<u32>() - 1;
        unsafe {
            pids.set_len(items_count);
        }

        Ok(pids)
    }
}

/// Returns the PIDs of the active processes that match the ProcType passed in
///
/// # Examples
///
/// Get the list of running process IDs using `listpids`
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid;
///
/// if let Ok(pids) = proc_pid::listpids(proc_pid::ProcType::ProcAllPIDS) {
///     println!("Found {} processes using listpids()", pids.len());
/// }
/// ```
#[cfg(target_os = "linux")]
pub fn listpids(proc_types: ProcType) -> Result<Vec<u32>> {
    match proc_types {
        ProcType::ProcAllPIDS => {
            let mut pids = Vec::<u32>::new();

            let proc_dir = fs::read_dir("/proc").map_err(|e| LibProcError::CantReadProcFS(format!("Could not read '/proc': {}", e)))?;

            for entry in proc_dir {
                let path = entry.map_err(|_| LibProcError::CantReadProcFS("Couldn't get /proc/ filename".into()))?.path();
                let filename = path.file_name();
                if let Some(name) = filename {
                    if let Some(n) = name.to_str() {
                        if let Ok(pid) = n.parse::<u32>() {
                            pids.push(pid);
                        }
                    }
                }
            }

            Ok(pids)
        }
        _ => Err(LibProcError::UnsupportedProcType.into())
    }
}

/// Search through the current processes looking for open file references which match
/// a specified path or volume.
#[cfg(target_os = "macos")]
pub fn listpidspath(proc_types: ProcType, path: &str) -> Result<Vec<u32>> {
    let c_path = CString::new(path).with_context(|| "Can't create CString")?;

    let buffer_size = unsafe {
        proc_listpidspath(proc_types as u32, 0, c_path.as_ptr() as * const c_char, 0, ptr::null_mut(), 0)
    };
    if buffer_size <= 0 {
        return Err(errno().into());
    }

    let capacity = buffer_size as usize / mem::size_of::<u32>();
    let mut pids: Vec<u32> = Vec::with_capacity(capacity);
    let buffer_ptr = pids.as_mut_ptr() as *mut c_void;

    let ret = unsafe {
        proc_listpidspath(
            proc_types as u32,
            0,
            c_path.as_ptr() as *const c_char,
            0,
            buffer_ptr,
            buffer_size as i32,
        )
    };

    if ret <= 0 {
        Err(errno().into())
    } else {
        let items_count = ret as usize / mem::size_of::<u32>() - 1;
        unsafe {
            pids.set_len(items_count);
        }

        Ok(pids)
    }
}

/// Get info about a process, task, thread or work queue by specifying the appropriate type for `T`:
/// - `BSDInfo`
/// - `TaskInfo`
/// - `TaskAllInfo`
/// - `ThreadInfo`
/// - `WorkQueueInfo`
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid::pidinfo;
/// use libproc::libproc::bsd_info::BSDInfo;
/// use std::process;
///
/// let pid = process::id() as i32;
///
/// // Get the `BSDInfo` for Process of pid 0
/// match pidinfo::<BSDInfo>(pid, 0) {
///     Ok(info) => assert_eq!(info.pbi_pid as i32, pid),
///     Err(err) => assert!(false, "Error retrieving process info: {}", err)
/// };
/// ```
#[cfg(target_os = "macos")]
pub fn pidinfo<T: PIDInfo>(pid: i32, arg: u64) -> Result<T> {
    let flavor = T::flavor() as i32;
    let buffer_size = mem::size_of::<T>() as i32;
    let mut pidinfo = unsafe { std::mem::zeroed() };
    let buffer_ptr = &mut pidinfo as *mut _ as *mut c_void;
    let ret: i32;

    unsafe {
        ret = proc_pidinfo(pid, flavor, arg, buffer_ptr, buffer_size);
    };

    if ret <= 0 {
        Err(errno().into())
    } else {
        Ok(pidinfo)
    }
}

/// pidinfo not implemented on linux - Pull Requests welcome - TODO
#[cfg(not(target_os = "macos"))]
pub fn pidinfo<T: PIDInfo>(_pid: i32, _arg: u64) -> Result<T> {
    unimplemented!()
}

/// Get the filename associated with a memory region
///
/// # Examples
///
/// ```
/// use libproc::libproc::proc_pid::regionfilename;
///
/// // This checks that it can find the regionfilename of the region at address 0, of the init process with PID 1
/// use libproc::libproc::proc_pid::am_root;
///
/// if am_root() {
///     match regionfilename(1, 0) {
///         Ok(regionfilename) => println!("Region Filename (at address = 0) of init process PID = 1 is '{}'", regionfilename),
///         Err(message) => panic!(message)
///     }
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn regionfilename(pid: i32, address: u64) -> Result<String> {
    let mut buf: Vec<u8> = Vec::with_capacity((PROC_PIDPATHINFO_MAXSIZE - 1) as _);
    let buffer_ptr = buf.as_mut_ptr() as *mut c_void;
    let buffer_size = buf.capacity() as u32;
    let ret: i32;

    unsafe {
        ret = proc_regionfilename(pid, address, buffer_ptr, buffer_size);
    };

    helpers::check_errno(ret, &mut buf)
}

/// Get the filename associated with a memory region
///
/// # Examples
///
/// ```
/// use libproc::libproc::proc_pid::regionfilename;
///
/// // This checks that it can find the regionfilename of the region at address 0, of the init process with PID 1
/// use libproc::libproc::proc_pid::am_root;
///
/// if am_root() {
///     match regionfilename(1, 0) {
///         Ok(regionfilename) => println!("Region Filename (at address = 0) of init process PID = 1 is '{}'", regionfilename),
///         Err(message) => panic!(message)
///     }
/// }
/// ```
#[cfg(not(target_os = "macos"))]
pub fn regionfilename(_pid: i32, _address: u64) -> Result<String> {
    Err(LibProcError::NotImplemented("'regionfilename' not implemented on linux").into())
}

/// Get the path of the executable file being run for a process
///
/// # Examples
///
/// ```
/// use libproc::libproc::proc_pid::pidpath;
///
/// match pidpath(1) {
///     Ok(path) => println!("Path of init process with PID = 1 is '{}'", path),
///     Err(message) => assert!(false, "{}", message)
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn pidpath(pid: i32) -> Result<String> {
    let mut buf: Vec<u8> = Vec::with_capacity((PROC_PIDPATHINFO_MAXSIZE - 1) as _);
    let buffer_ptr = buf.as_mut_ptr() as *mut c_void;
    let buffer_size = buf.capacity() as u32;
    let ret: i32;

    unsafe {
        ret = proc_pidpath(pid, buffer_ptr, buffer_size as _);
    };

    helpers::check_errno(ret, &mut buf)
}

/// Get the path of the executable file being run for a process
///
/// # Examples
///
/// ```
/// use libproc::libproc::proc_pid::{pidpath, am_root};
///
/// match pidpath(1) {
///     Ok(path) => println!("Path of init process with PID = 1 is '{}'", path),
///     Err(_) if !am_root() => println!("pidpath() needs to be run as root"),
///     Err(message) if am_root() => assert!(false, "{}", message),
///     _ => assert!(false, "Unknown error")
/// }
/// ```
#[cfg(target_os = "linux")]
pub fn pidpath(pid: i32) -> Result<String> {
    let exe_path = CString::new(format!("/proc/{}/exe", pid))
        .with_context(|| "Could not create CString")?;
    let mut buf: Vec<u8> = Vec::with_capacity(PATH_MAX as usize - 1);
    let buffer_ptr = buf.as_mut_ptr() as *mut c_char;
    let buffer_size = buf.capacity();
    let ret = unsafe {
        readlink(exe_path.as_ptr(), buffer_ptr, buffer_size)
    };

    helpers::check_errno(ret as i32, &mut buf)
}

/// Get the major and minor version numbers of the native libproc library (Mac OS X)
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid;
///
/// match proc_pid::libversion() {
///     Ok((major, minor)) => println!("Libversion: {}.{}", major, minor),
///     Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn libversion() -> Result<(i32, i32)> {
    let mut major = 0;
    let mut minor = 0;
    let ret: i32;

    unsafe {
        ret = proc_libversion(&mut major, &mut minor);
    };

    // return value of 0 indicates success (inconsistent with other functions... :-( )
    if ret == 0 {
        Ok((major, minor))
    } else {
        Err(errno().into())
    }
}

/// Get the major and minor version numbers of the native libproc library (Mac OS X)
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid;
///
/// match proc_pid::libversion() {
///     Ok((major, minor)) => println!("Libversion: {}.{}", major, minor),
///     Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
/// }
/// ```
#[cfg(not(target_os = "macos"))]
pub fn libversion() -> Result<(i32, i32)> {
    Err(LibProcError::NotImplemented("Linux does not use a library, so no library version number").into())
}

/// Get the name of a process
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid;
///
/// match proc_pid::name(1) {
///     Ok(name) => println!("Name: {}", name),
///     Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn name(pid: i32) -> Result<String> {
    let mut namebuf: Vec<u8> = Vec::with_capacity((PROC_PIDPATHINFO_MAXSIZE - 1) as _);
    let buffer_ptr = namebuf.as_ptr() as *mut c_void;
    let buffer_size = namebuf.capacity() as u32;
    let ret: i32;

    unsafe {
        ret = proc_name(pid, buffer_ptr, buffer_size);
    };

    if ret <= 0 {
        Err(errno().into())
    } else {
        unsafe {
            namebuf.set_len(ret as usize);
        }

        String::from_utf8(namebuf).map_err(|e| e.into()).into()
    }
}


/// Get the name of a Process using it's Pid
#[cfg(target_os = "linux")]
pub fn name(pid: i32) -> Result<String> {
    helpers::procfile_field(&format!("/proc/{}/status", pid), "Name")
}

/// Get information on all running processes.
///
/// `max_len` is the maximum number of array to return.
/// The length of return value: `Vec<T::Item>` may be less than `max_len`.
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid::{listpidinfo, pidinfo};
/// use libproc::libproc::task_info::TaskAllInfo;
/// use libproc::libproc::file_info::{ListFDs, ProcFDType};
/// use std::process;
///
/// let pid = process::id() as i32;
///
/// if let Ok(info) = pidinfo::<TaskAllInfo>(pid, 0) {
///     if let Ok(fds) = listpidinfo::<ListFDs>(pid, info.pbsd.pbi_nfiles as usize) {
///         for fd in &fds {
///             let fd_type = ProcFDType::from(fd.proc_fdtype);
///             println!("File Descriptor: {}, Type: {:?}", fd.proc_fd, fd_type);
///         }
///     }
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn listpidinfo<T: ListPIDInfo>(pid: i32, max_len: usize) -> Result<Vec<T::Item>> {
    let flavor = T::flavor() as i32;
    let buffer_size = mem::size_of::<T::Item>() as i32 * max_len as i32;
    let mut buffer = Vec::<T::Item>::with_capacity(max_len);
    let buffer_ptr = unsafe {
        buffer.set_len(max_len);
        buffer.as_mut_ptr() as *mut c_void
    };

    let ret: i32;

    unsafe {
        ret = proc_pidinfo(pid, flavor, 0, buffer_ptr, buffer_size);
    };

    if ret <= 0 {
        Err(errno().into())
    } else {
        let actual_len = ret as usize / mem::size_of::<T::Item>();
        buffer.truncate(actual_len);
        Ok(buffer)
    }
}

/// listpidinfo is not implemented on Linux - Pull Requests welcome - TODO
#[cfg(not(target_os = "macos"))]
pub fn listpidinfo<T: ListPIDInfo>(_pid: i32, _max_len: usize) -> Result<Vec<T::Item>> {
    unimplemented!()
}

#[cfg(target_os = "macos")]
/// Gets the path of current working directory for the process with the provided pid.
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid::pidcwd;
///
/// match pidcwd(1) {
///     Ok(cwd) => println!("The CWD of the process with pid=1 is '{}'", cwd.display()),
///     Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
/// }
/// ```
pub fn pidcwd(_pid: pid_t) -> Result<PathBuf> {
    Err(LibProcError::NotImplemented("pidcwd is not implemented for macos").into())
}

#[cfg(target_os = "linux")]
/// Gets the path of current working directory for the process with the provided pid.
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid::pidcwd;
///
/// match pidcwd(1) {
///     Ok(cwd) => println!("The CWD of the process with pid=1 is '{}'", cwd.display()),
///     Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
/// }
/// ```
pub fn pidcwd(pid: pid_t) -> Result<PathBuf> {
    fs::read_link(format!("/proc/{}/cwd", pid)).with_context(|| "Can't read link")
}

/// Gets path of current working directory for the current process.
///
/// Just wraps rusts env::current_dir() function so not so useful.
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid::cwdself;
///
/// match cwdself() {
///     Ok(cwd) => println!("The CWD of the current process is '{}'", cwd.display()),
///     Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
/// }
/// ```
pub fn cwdself() -> Result<PathBuf> {
    env::current_dir().map_err(|e| LibProcError::GenericError(e.to_string()).into())
}

/// Determine if the current user ID of this process is root
///
/// # Examples
///
/// ```
/// use libproc::libproc::proc_pid::am_root;
///
/// if am_root() {
///     println!("With great power comes great responsibility");
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn am_root() -> bool {
    // geteuid() is unstable still - wait for it or wrap this:
    // https://stackoverflow.com/questions/3214297/how-can-my-c-c-application-determine-if-the-root-user-is-executing-the-command
    unsafe { libc::getuid() == 0 }
}

/// Return true if the calling process is being run by the root user, false otherwise
#[cfg(target_os = "linux")]
pub fn am_root() -> bool {
    // when this becomes stable in rust libc then we can remove this function or combine for mac and linux
    unsafe { libc::geteuid() == 0 }
}

// run tests with 'cargo test -- --nocapture' to see the test output
#[cfg(test)]
mod test {
    #[cfg(target_os = "linux")]
    use std::process;
    use std::env;
    use errno::Errno;

    #[cfg(target_os = "macos")]
    use crate::libproc::bsd_info::BSDInfo;
    #[cfg(target_os = "macos")]
    use crate::libproc::file_info::ListFDs;
    #[cfg(target_os = "macos")]
    use crate::libproc::task_info::TaskAllInfo;

    #[cfg(target_os = "macos")]
    use super::{libversion, listpidinfo, ListThreads, pidinfo};
    use super::{name, cwdself, listpids, pidpath};
    #[cfg(target_os = "linux")]
    use super::pidcwd;
    use crate::libproc::proc_pid::ProcType;
    use super::am_root;
    #[cfg(target_os = "linux")]
    use crate::libproc::helpers;
    #[cfg(target_os = "macos")]
    use crate::libproc::task_info::TaskInfo;
    #[cfg(target_os = "macos")]
    use crate::libproc::thread_info::ThreadInfo;
    #[cfg(target_os = "macos")]
    use crate::libproc::work_queue_info::WorkQueueInfo;
    use crate::libproc::error::LibProcError;

    #[cfg(target_os = "macos")]
    #[test]
    fn pidinfo_test() {
        use std::process;
        let pid = process::id() as i32;

        match pidinfo::<BSDInfo>(pid, 0) {
            Ok(info) => assert_eq!(info.pbi_pid as i32, pid),
            Err(e) => panic!("Error retrieving BSDInfo: {}", e)
        };
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn taskinfo_test() {
        use std::process;
        let pid = process::id() as i32;

        match pidinfo::<TaskInfo>(pid, 0) {
            Ok(info) => assert!(info.pti_virtual_size > 0),
            Err(e) => panic!("Error retrieving TaskInfo: {}", e)
        };
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn taskallinfo_test() {
        use std::process;
        let pid = process::id() as i32;

        match pidinfo::<TaskAllInfo>(pid, 0) {
            Ok(info) => assert!(info.ptinfo.pti_virtual_size > 0),
            Err(e) => panic!("Error retrieving TaskAllInfo: {}", e)
        };
    }

    #[ignore]
    #[cfg(target_os = "macos")]
    #[test]
    fn threadinfo_test() {
        use std::process;
        let pid = process::id() as i32;

        match pidinfo::<ThreadInfo>(pid, 0) {
            Ok(info) => assert!(info.pth_user_time > 0),
            Err(e) => panic!("Error retrieving ThreadInfo: {}", e)
        };
    }

    #[ignore]
    #[cfg(target_os = "macos")]
    #[test]
    fn workqueueinfo_test() {
        use std::process;
        let pid = process::id() as i32;

        match pidinfo::<WorkQueueInfo>(pid, 0) {
            Ok(info) => assert!(info.pwq_nthreads > 0),
            Err(_) => panic!("Error retrieving WorkQueueInfo")
        };
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn listpidinfo_test() {
        use std::process;
        let pid = process::id() as i32;

        if let Ok(info) = pidinfo::<TaskAllInfo>(pid, 0) {
            if let Ok(threads) = listpidinfo::<ListThreads>(pid, info.ptinfo.pti_threadnum as usize) {
                assert!(!threads.is_empty());
            }
            if let Ok(fds) = listpidinfo::<ListFDs>(pid, info.pbsd.pbi_nfiles as usize) {
                assert!(!fds.is_empty());
            };
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn libversion_test() {
        libversion().expect("libversion() failed");
    }

    #[test]
    fn listpids_test() {
        let pids = listpids(ProcType::ProcAllPIDS).expect("Could not list pids");
        assert!(pids.len() > 1);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn listpids_invalid_type_test() {
        assert!(listpids(ProcType::ProcPGRPOnly).is_err());
    }

    #[test]
    fn name_test() {
        #[cfg(target_os = "linux")]
            let expected_name = "systemd";
        #[cfg(target_os = "macos")]
            let expected_name = "launchd";

        if am_root() || cfg!(target_os = "linux") {
            match name(1) {
                Ok(name) => assert_eq!(expected_name, name),
                Err(_) => panic!("Error retrieving process name")
            }
        } else {
            println!("Cannot run 'name_test' on macos unless run as root");
        }
    }

    #[test]
    // This checks that it cannot find the path of the process with pid -1 and returns correct error message
    fn pidpath_test_unknown_pid_test() {
        match pidpath(-1) {
            Ok(path) => panic!("It found the path of process with ID = -1 (path = {}), that's not possible\n", path),
            #[cfg(target_os = "macos")]
            Err(e) => assert!(matches!(e.downcast::<LibProcError>().unwrap(), LibProcError::OSError(Errno(3)))),
            #[cfg(target_os = "linux")]
            Err(e) => assert!(matches!(e.downcast::<LibProcError>().unwrap(), LibProcError::OSError(Errno(2)))),
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    // This checks that it cannot find the path of the process with pid 1
    fn pidpath_test() {
        assert_eq!("/sbin/launchd", pidpath(1).expect("pidpath() failed"));
    }

    // Pretty useless test as it uses the exact same code as the function - but I guess we
    // should check it can be called and returns correct value
    #[test]
    fn cwd_self_test() {
        assert_eq!(env::current_dir().expect("Could not get current directory"),
                   cwdself().expect("cwdself() failed"));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn pidcwd_of_self_test() {
        assert_eq!(env::current_dir().expect("Could not get current directory"),
                   pidcwd(process::id() as i32).expect("pidcwd() failed"));
    }

    #[test]
    fn am_root_test() {
        if am_root() {
            println!("You are root");
        } else {
            println!("You are not root");
        }
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn procfile_field_test() {
        if am_root() {
            assert!(helpers::procfile_field("/proc/1/status", "invalid").is_err());
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn listpidspath_test() {
        let pids = super::listpidspath(ProcType::ProcAllPIDS, "/")
            .expect("listpidspath() failed");
        assert!(pids.len() > 1);
    }
}
