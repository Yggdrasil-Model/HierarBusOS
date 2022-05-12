#![no_std]
#[macro_use]
extern crate alloc;
pub mod socket;
pub use socket::*;
#[macro_use]
extern crate console_support;
#[macro_use]
extern crate num_derive;
extern crate num;
extern crate lazy_static;
use core::{fmt};
#[macro_use]
extern crate log;
#[allow(dead_code)]
#[repr(isize)]
#[derive(Debug, FromPrimitive)]
pub enum SysError {
    EUNDEF = 0,
    EPERM = 1,
    ENOENT = 2,
    ESRCH = 3,
    EINTR = 4,
    EIO = 5,
    ENXIO = 6,
    E2BIG = 7,
    ENOEXEC = 8,
    EBADF = 9,
    ECHILD = 10,
    EAGAIN = 11,
    ENOMEM = 12,
    EACCES = 13,
    EFAULT = 14,
    ENOTBLK = 15,
    EBUSY = 16,
    EEXIST = 17,
    EXDEV = 18,
    ENODEV = 19,
    ENOTDIR = 20,
    EISDIR = 21,
    EINVAL = 22,
    ENFILE = 23,
    EMFILE = 24,
    ENOTTY = 25,
    ETXTBSY = 26,
    EFBIG = 27,
    ENOSPC = 28,
    ESPIPE = 29,
    EROFS = 30,
    EMLINK = 31,
    EPIPE = 32,
    EDOM = 33,
    ERANGE = 34,
    EDEADLK = 35,
    ENAMETOOLONG = 36,
    ENOLCK = 37,
    ENOSYS = 38,
    ENOTEMPTY = 39,
    ELOOP = 40,
    EIDRM = 43,
    ENOTSOCK = 80,
    ENOPROTOOPT = 92,
    EPFNOSUPPORT = 96,
    EAFNOSUPPORT = 97,
    ENOBUFS = 105,
    EISCONN = 106,
    ENOTCONN = 107,
    ETIMEDOUT = 110,
    ECONNREFUSED = 111,
}

#[allow(non_snake_case)]
impl fmt::Display for SysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::SysError::*;
        write!(
            f,
            "{}",
            match self {
                EPERM => "Operation not permitted",
                ENOENT => "No such file or directory",
                ESRCH => "No such process",
                EINTR => "Interrupted system call",
                EIO => "I/O error",
                ENXIO => "No such device or address",
                E2BIG => "Argument list too long",
                ENOEXEC => "Exec format error",
                EBADF => "Bad file number",
                ECHILD => "No child processes",
                EAGAIN => "Try again",
                ENOMEM => "Out of memory",
                EACCES => "Permission denied",
                EFAULT => "Bad address",
                ENOTBLK => "Block device required",
                EBUSY => "Device or resource busy",
                EEXIST => "File exists",
                EXDEV => "Cross-device link",
                ENODEV => "No such device",
                ENOTDIR => "Not a directory",
                EISDIR => "Is a directory",
                EINVAL => "Invalid argument",
                ENFILE => "File table overflow",
                EMFILE => "Too many open files",
                ENOTTY => "Not a typewriter",
                ETXTBSY => "Text file busy",
                EFBIG => "File too large",
                ENOSPC => "No space left on device",
                ESPIPE => "Illegal seek",
                EROFS => "Read-only file system",
                EMLINK => "Too many links",
                EPIPE => "Broken pipe",
                EDOM => "Math argument out of domain of func",
                ERANGE => "Math result not representable",
                EDEADLK => "Resource deadlock would occur",
                ENAMETOOLONG => "File name too long",
                ENOLCK => "No record locks available",
                ENOSYS => "Function not implemented",
                ENOTEMPTY => "Directory not empty",
                ELOOP => "Too many symbolic links encountered",
                ENOTSOCK => "Socket operation on non-socket",
                ENOPROTOOPT => "Protocol not available",
                EPFNOSUPPORT => "Protocol family not supported",
                EAFNOSUPPORT => "Address family not supported by protocol",
                ENOBUFS => "No buffer space available",
                EISCONN => "Transport endpoint is already connected",
                ENOTCONN => "Transport endpoint is not connected",
                ECONNREFUSED => "Connection refused",
                _ => "Unknown error",
            },
        )
    }
}

#[macro_export]
macro_rules! enum_with_unknown {
    (
        $( #[$enum_attr:meta] )*
        pub enum $name:ident($ty:ty) {
            $( $variant:ident = $value:expr ),+ $(,)*
        }
    ) => {
        enum_with_unknown! {
            $( #[$enum_attr] )*
            pub doc enum $name($ty) {
                $( #[doc(shown)] $variant = $value ),+
            }
        }
    };
    (
        $( #[$enum_attr:meta] )*
        pub doc enum $name:ident($ty:ty) {
            $(
              $( #[$variant_attr:meta] )+
              $variant:ident = $value:expr $(,)*
            ),+
        }
    ) => {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        $( #[$enum_attr] )*
        pub enum $name {
            $(
              $( #[$variant_attr] )*
              $variant
            ),*,
            Unknown($ty)
        }

        impl ::core::convert::From<$ty> for $name {
            fn from(value: $ty) -> Self {
                match value {
                    $( $value => $name::$variant ),*,
                    other => $name::Unknown(other)
                }
            }
        }

        impl ::core::convert::From<$name> for $ty {
            fn from(value: $name) -> Self {
                match value {
                    $( $name::$variant => $value ),*,
                    $name::Unknown(other) => other
                }
            }
        }
    }
}

