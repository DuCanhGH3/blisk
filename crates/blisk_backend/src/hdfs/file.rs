use super::{bindgen, AppHdfs};
use std::{
    ffi::{c_void, CStr},
    io::{Error as IoError, Read, Result as IoResult, Seek, SeekFrom, Write},
};

pub struct FileInfo {
    pub kind: u32,
    pub name: String,
    pub last_mod: i64,
    pub size: i64,
    pub replication: i16,
    pub block_size: i64,
    pub owner: String,
    pub group: String,
    pub permissions: i16,
    pub last_access: i64,
}

impl From<bindgen::hdfsFileInfo> for FileInfo {
    fn from(value: bindgen::hdfsFileInfo) -> Self {
        let c_name = unsafe { CStr::from_ptr(value.mName) };
        let c_owner = unsafe { CStr::from_ptr(value.mOwner) };
        let c_group = unsafe { CStr::from_ptr(value.mGroup) };
        FileInfo {
            kind: value.mKind,
            name: c_name
                .to_str()
                .expect("Filename is not valid UTF-8!")
                .to_owned(),
            last_mod: value.mLastMod,
            size: value.mSize,
            replication: value.mReplication,
            block_size: value.mBlockSize,
            owner: c_owner
                .to_str()
                .expect("Owner is not valid UTF-8!")
                .to_owned(),
            group: c_group
                .to_str()
                .expect("Group is not valid UTF-8!")
                .to_owned(),
            permissions: value.mPermissions,
            last_access: value.mLastAccess,
        }
    }
}

#[derive(Clone)]
pub struct File<'a> {
    pub(super) hdfs: &'a AppHdfs,
    pub(super) file: bindgen::hdfsFile,
    pub(super) path: String,
}

unsafe impl Send for File<'_> {}

unsafe impl Sync for File<'_> {}

impl File<'_> {
    pub fn at(&self, buf: &mut [u8], offset: u64) -> IoResult<usize> {
        let n = unsafe {
            bindgen::hdfsPread(
                self.hdfs.fs,
                self.file,
                offset as i64,
                buf.as_ptr() as *mut c_void,
                buf.len() as i32,
            )
        };

        if n == -1 {
            return Err(IoError::last_os_error());
        }

        Ok(n as usize)
    }

    fn seek_inner(&self, offset: i64) -> IoResult<()> {
        let n = unsafe { bindgen::hdfsSeek(self.hdfs.fs, self.file, offset) };

        if n == -1 {
            return Err(IoError::last_os_error());
        }

        Ok(())
    }

    fn tell_inner(&self) -> IoResult<i64> {
        let n = unsafe { bindgen::hdfsTell(self.hdfs.fs, self.file) };

        if n == -1 {
            return Err(IoError::last_os_error());
        }

        Ok(n)
    }
}

impl Read for File<'_> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let n = unsafe {
            bindgen::hdfsRead(
                self.hdfs.fs,
                self.file,
                buf.as_ptr() as *mut c_void,
                buf.len() as i32,
            )
        };

        if n == -1 {
            return Err(IoError::last_os_error());
        }

        Ok(n as usize)
    }
}

impl Seek for File<'_> {
    fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
        match pos {
            SeekFrom::Start(n) => {
                self.seek_inner(n as i64)?;
                Ok(n)
            }
            SeekFrom::Current(n) => {
                let current = self.tell_inner()?;
                let offset = (current + n) as u64;
                self.seek_inner(offset as i64)?;
                Ok(offset)
            }
            SeekFrom::End(n) => {
                let meta = self.hdfs.read(&self.path)?;
                let offset = meta.size as i64 + n;
                self.seek_inner(offset)?;
                Ok(offset as u64)
            }
        }
    }
}

impl Write for File<'_> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        let n = unsafe {
            bindgen::hdfsWrite(
                self.hdfs.fs,
                self.file,
                buf.as_ptr() as *const c_void,
                buf.len() as i32,
            )
        };

        if n == -1 {
            return Err(IoError::last_os_error());
        }

        Ok(n as usize)
    }

    fn flush(&mut self) -> IoResult<()> {
        let n = unsafe { bindgen::hdfsFlush(self.hdfs.fs, self.file) };

        if n == -1 {
            return Err(IoError::last_os_error());
        }

        Ok(())
    }
}
