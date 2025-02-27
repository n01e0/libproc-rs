//! `procinfo` is a simple program to demonstrate the use of the [`libproc`](../libproc/index.html)
//! library functions.
//!
//! It prints out info about a process specified by its pid, or the current process if no pid
//! specified.
//!
//! Usage
//! =
//!```
//! procinfo [pid]
//!
//!```
//!
//! Which will produce output similar to:
//! ```
//! Libversion: 1.1
//! Pid: 8484
//! Path: /Users/andrew/workspace/libproc-rs/target/debug/procinfo
//! Name: procinfo
//! Region Filename (at address 0): /Users/andrew/workspace/libproc-rs/target/debug/procinfo
//! There are currently 454 processes active
//! 8496
//! ...
//! ```

extern crate libproc;
extern crate libc;

use std::env;
use std::io::Write;
use libproc::libproc::proc_pid;
use libproc::libproc::pid_rusage::{pidrusage, RUsageInfoV0};

mod c {
    extern crate libc;
    extern {
        pub fn getpid() -> libc::pid_t;
    }
}

fn getpid() -> i32 {
    unsafe {
        c::getpid()
    }
}

fn procinfo(pid: i32) {
    match proc_pid::libversion() {
        Ok((major, minor)) => println!("Libversion: {}.{}", major, minor),
        Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
    }

    println!("Pid: {}", pid);

    match proc_pid::pidpath(pid) {
        Ok(path) => println!("Path: {}", path),
        Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
    }

    match pidrusage::<RUsageInfoV0>(pid) {
        Ok(resource_usage) => println!("Memory Used: {} Bytes", resource_usage.ri_resident_size),
        Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
    }

    match proc_pid::name(pid) {
        Ok(name) => println!("Name: {}", name),
        Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
    }

    match proc_pid::regionfilename(pid, 0) {
        Ok(regionfilename) => println!("Region Filename (at address 0): {}", regionfilename),
        Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
    }

    match proc_pid::listpids(proc_pid::ProcType::ProcAllPIDS) {
        Ok(pids) => {
            println!("There are currently {} processes active", pids.len());
            for pid in pids {
                println!("{}", pid);
            }
        },
        Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let pid = if args.len() == 1 {
       getpid()
    } else {
        args[1].clone().parse::<i32>().unwrap()
    };

    procinfo(pid);
}