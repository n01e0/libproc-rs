use crate::errno::errno;
use crate::libproc::error::LibProcError;
use anyhow::{Context, Result};
#[cfg(target_os = "linux")]
use std::fs::File;
#[cfg(target_os = "linux")]
use std::io::{BufRead, BufReader};

/// Helper function to get errno and return a String with the passed in return_code, the error
/// number and a possible message
#[cfg(target_os = "macos")]
pub fn get_errno_with_message(return_code: i32) -> String {
    let e = errno();
    let code = e.0 as i32;
    format!("return code = {}, errno = {}, message = '{}'", return_code, code, e)
}

/// Helper function that depending on the `ret` value:
/// - is negative or 0, then form an error message from the `errno` value
/// - is positive, take `ret` as the length of the success message in `buf` in bytes
pub fn check_errno(ret: i32, buf: &mut Vec<u8>) -> Result<String> {
    if ret <= 0 {
        Err(errno().into())
    } else {
        unsafe {
            buf.set_len(ret as usize);
        }

        String::from_utf8(buf.to_vec()).map_err(|e| e.into())
    }
}

#[cfg(target_os = "linux")]
/// A helper function for finding named fields in specific /proc FS files for processes
/// This will be more useful when implementing more linux functions
pub fn procfile_field(filename: &str, field_name: &str) -> Result<String> {
    const SEPARATOR: &str = ":";
    let line_header = format!("{}{}", field_name, SEPARATOR);

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).with_context(|| format!("Could not open /proc file '{}'", filename))?;
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let line = line.with_context(|| "Could not read file contents")?;
        if line.starts_with(&line_header) {
            let parts: Vec<&str> = line.split(SEPARATOR).collect();
            return Ok(parts[1].trim().to_owned());
        }
    }

    Err(LibProcError::CouldNotFoundProc(field_name.into(), filename.into()).into())
}

#[cfg(target_os = "linux")]
/// Parse a memory amount string into integer number of bytes
/// e.g. 220844 kB -->
pub fn parse_memory_string(line: &str) -> Result<u64> {
    let parts: Vec<&str> = line.trim().split(' ').collect();
    if parts.is_empty() {
        return Err(LibProcError::CouldNotParseMemoryString(line.into()).into())
    }
    let multiplier: u64 = if parts.len() == 2 {
        match parts[1] {
            "MB" => 1024 * 1024,
            "kB" => 1024,
            "B" => 1,
            _ => return Err(LibProcError::CouldNotParseMemoryString(line.into()).into())
        }
    } else {
        1
    };

    let value:u64 = parts[0].parse().map_err(|_| LibProcError::CouldNotParseMemoryString(line.into()))?;

    Ok(value * multiplier)
}

#[cfg(test)]
mod test {
    use crate::{errno::{set_errno, Errno}, libproc::error::LibProcError};
    use super::check_errno;

    #[cfg(target_os = "linux")]
    mod linux {
        use crate::libproc::helpers::parse_memory_string;

        #[test]
        fn test_valid_memory_string() {
            let res = parse_memory_string("220844 kB");
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), 226144256);
        }

        #[test]
        fn test_valid_memory_string_spaces() {
            let res = parse_memory_string("  220844 kB  ");
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), 226144256);
        }

        #[test]
        fn test_invalid_memory_string_units() {
            assert!(parse_memory_string("  220844 THz  ").is_err());
        }

        #[test]
        fn test_invalid_memory_string() {
            assert!(parse_memory_string("    ").is_err());
        }

        #[test]
        fn test_invalid_memory_string_empty() {
            assert!(parse_memory_string("gobble dee gook").is_err())
        }
    }

    #[test]
    fn invalid_utf8() {
        let mut buf: Vec<u8> = vec!(255, 0, 0);

        // Test
        if let Err(e) = check_errno(buf.len() as i32, &mut buf) {
            assert!(matches!(e.downcast::<LibProcError>().unwrap(), LibProcError::OSError(Errno(1))));
        }
    }

    #[test]
    fn positive_ret() {
        let message = "custom message";
        let mut buf: Vec<u8> = Vec::from(message.as_bytes());

        // Test
        if let Ok(msg) = check_errno(buf.len() as i32, &mut buf) {
            assert_eq!(msg, message);
        }
    }

    #[test]
    fn negative_ret() {
        let mut buf: Vec<u8> = vec!();
        set_errno(Errno(-1));

        // Test
        if let Err(e) = check_errno(-1, &mut buf) {
            #[cfg(target_os = "macos")]
            assert!(matches!(e.downcast::<LibProcError>(), Ok(LibProcError::OSError(Errno(-1)))));
            #[cfg(target_os = "linux")]
            assert!(matches!(e.downcast::<LibProcError>(), Ok(LibProcError::OSError(Errno(-1)))));
        }
    }

    #[test]
    fn zero_ret() {
        let mut buf: Vec<u8> = vec!();
        set_errno(Errno(2));

        // Test
        if let Err(e) = check_errno(0, &mut buf) {
            assert!(matches!(e.downcast::<LibProcError>(), Ok(LibProcError::OSError(Errno(2)))));
        }
    }
}
