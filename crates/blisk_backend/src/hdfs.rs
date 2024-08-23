use std::ffi::{CStr, CString};

use crate::utils::os::clear_errno;

use super::bindgen;

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

pub struct AppHdfs {
    fs: bindgen::hdfsFS,
}

unsafe impl Sync for AppHdfs {}

unsafe impl Send for AppHdfs {}

impl Drop for AppHdfs {
    fn drop(&mut self) {
        unsafe { bindgen::hdfsDisconnect(self.fs) };
    }
}

impl AppHdfs {
    pub fn new(name_node: &str, user: &str) -> std::io::Result<AppHdfs> {
        let nn = CString::new(name_node)?;
        let user = CString::new(user)?;
        let fs = unsafe {
            let builder = bindgen::hdfsNewBuilder();
            bindgen::hdfsBuilderSetNameNode(builder, nn.as_ptr());
            bindgen::hdfsBuilderSetUserName(builder, user.as_ptr());
            bindgen::hdfsBuilderConnect(builder)
        };
        if fs.is_null() {
            return Err(std::io::Error::last_os_error());
        }
        Ok(AppHdfs { fs })
    }
    pub fn read(&self, path: &str) -> std::io::Result<FileInfo> {
        let path = CString::new(path)?;

        let file_raw = unsafe { bindgen::hdfsGetPathInfo(self.fs, path.as_ptr()) };

        if file_raw.is_null() {
            return Err(std::io::Error::last_os_error());
        }

        let file = unsafe {
            let file_info = FileInfo::from(*file_raw);
            bindgen::hdfsFreeFileInfo(file_raw, 1);
            file_info
        };

        Ok(file)
    }
    pub fn readdir(&self, path: &str) -> std::io::Result<Vec<FileInfo>> {
        // Since `hdfsListDirectory` will only set `errno` if it fails to
        // read directory but return `NULL` for both when it does so and when
        // it reads an empty directory, we have to clear the value out first.
        clear_errno();

        let path = CString::new(path)?;

        let mut entries = 0;

        let dir_raw = unsafe { bindgen::hdfsListDirectory(self.fs, path.as_ptr(), &mut entries) };

        if dir_raw.is_null() {
            let err = std::io::Error::last_os_error();

            return match err.raw_os_error() {
                None => Ok(Vec::new()),
                Some(0) => Ok(Vec::new()),
                _ => Err(err),
            };
        }

        let mut dir = Vec::with_capacity(entries as usize);

        for i in 0..entries as isize {
            let m = unsafe { FileInfo::from(*dir_raw.offset(i)) };

            dir.push(m)
        }

        unsafe { bindgen::hdfsFreeFileInfo(dir_raw, entries) };

        Ok(dir)
    }
    pub fn mkdir(&self, path: &str) -> std::io::Result<()> {
        let path = CString::new(path)?;

        let status = unsafe { bindgen::hdfsCreateDirectory(self.fs, path.as_ptr()) };

        if status == -1 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(())
    }
    pub fn mv(&self, old: &str, new: &str) -> std::io::Result<()> {
        let old = CString::new(old)?;
        let new = CString::new(new)?;

        let status = unsafe { bindgen::hdfsRename(self.fs, old.as_ptr(), new.as_ptr()) };

        if status == -1 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(())
    }
    pub fn rm(&self, path: &str) -> std::io::Result<()> {
        let path = CString::new(path)?;

        let status = unsafe { bindgen::hdfsDelete(self.fs, path.as_ptr(), false.into()) };

        if status == -1 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(())
    }
    pub fn rmdir(&self, path: &str, recursive: Option<bool>) -> std::io::Result<()> {
        let path = CString::new(path)?;
        let recursive = recursive.unwrap_or(false);

        let status = unsafe { bindgen::hdfsDelete(self.fs, path.as_ptr(), recursive.into()) };

        if status == -1 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(())
    }
}
