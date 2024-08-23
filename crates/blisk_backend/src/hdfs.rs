use std::ffi::CString;

use super::bindgen;

pub struct HdfsFs(bindgen::hdfsFS);

unsafe impl Sync for HdfsFs {}

unsafe impl Send for HdfsFs {}

pub struct AppHdfs {
    inner: HdfsFs,
}

impl Drop for AppHdfs {
    fn drop(&mut self) {
        unsafe { bindgen::hdfsDisconnect(self.inner.0) };
    }
}

impl AppHdfs {
    pub fn new(name_node: &str, user: &str) -> AppHdfs {
        let nn = CString::new(name_node).unwrap();
        let user = CString::new(user).unwrap();
        let fs = unsafe {
            let builder = bindgen::hdfsNewBuilder();
            bindgen::hdfsBuilderSetNameNode(builder, nn.as_ptr());
            bindgen::hdfsBuilderSetUserName(builder, user.as_ptr());
            bindgen::hdfsBuilderConnect(builder)
        };
        if fs.is_null() {
            panic!("Failed to connect to HDFS: NULL pointer returned.");
        }
        let inner = HdfsFs(fs);
        AppHdfs { inner }
    }
}
