use crate::ffi::FFIString;

#[repr(C)]
#[no_mangle]
pub struct DownloadConfig {
    pub url: FFIString,
    pub path: FFIString,
    pub retry_times: u8,
    pub chunk_download: bool,
    pub version: i64,
    pub chunk_size: u64,
    pub timeout: u64,
}

#[no_mangle]
pub extern "C" fn DownloadConfig_Extern(config: DownloadConfig) {

}