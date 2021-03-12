use crate::ffi::library::OSPathChar;
use std::ffi::OsStr;

/// Trait for encoding into the platforms preferred encoding.
pub trait ToOsPathBuff {
    /// Creates an owned buffer with the platforms preferred encoding.
    fn to_os_path_buff(&self) -> Vec<OSPathChar>;

    /// Like `to_os_path_buff`, but with an additional null terminator.
    fn to_os_path_buff_null(&self) -> Vec<OSPathChar>;
}

impl<T> ToOsPathBuff for T
where
    T: AsRef<OsStr>,
{
    #[inline]
    #[cfg(windows)]
    fn to_os_path_buff(&self) -> Vec<OSPathChar> {
        use std::os::windows::ffi::OsStrExt;
        self.as_ref().encode_wide().collect()
    }

    #[inline]
    #[cfg(unix)]
    fn to_os_path_buff(&self) -> Vec<OSPathChar> {
        use std::os::unix::ffi::OsStrExt;
        Vec::from(self.as_ref().as_bytes())
    }

    #[inline]
    fn to_os_path_buff_null(&self) -> Vec<OSPathChar> {
        let mut buff = self.to_os_path_buff();
        buff.push(0);
        buff
    }
}
