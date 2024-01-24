use std::ffi::{c_char, CString};
use downloader_rs::download_configuration::DownloadConfiguration;
use downloader_rs::download_operation::DownloadOperation;
use downloader_rs::download_service::DownloadService;
use crate::ffi_string::FFIString;

#[repr(C)]
pub struct DownloadConfig {
    url: FFIString,
    path: FFIString,
    retry_times: u8,
    chunk_download: bool,
    version: i64,
    chunk_size: u64,
    timeout: u64,
}

#[repr(C)]
pub struct FFIDownloadService {}

#[repr(C)]
pub struct FFIDownloadOperation {}

#[no_mangle]
pub extern "C" fn start_download_service(multi_thread: bool, thread_count: u16, parallel_count: u16) -> *mut FFIDownloadService {
    let mut download_service = DownloadService::new()
        .set_worker_thread_count(thread_count as usize)
        .set_multi_thread(multi_thread);
    download_service.set_parallel_count(parallel_count as usize);
    download_service.start_service();
    Box::into_raw(Box::new(download_service)) as *mut FFIDownloadService
}

#[no_mangle]
pub extern "C" fn set_parallel_count(ptr: *mut FFIDownloadService, count: u16) {
    let ptr = ptr as *mut DownloadService;
    let service = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    service.set_parallel_count(count as usize)
}

#[no_mangle]
pub extern "C" fn stop_download_service(ptr: *mut FFIDownloadService) {
    let ptr = ptr as *mut DownloadService;
    if ptr.is_null() {
        return;
    }
    unsafe {
        let download_service = Box::from_raw(ptr);
        download_service.stop();
    }
}

#[no_mangle]
pub extern "C" fn add_downloader(ptr: *mut FFIDownloadService, config: DownloadConfig) -> *mut FFIDownloadOperation {
    let ptr = ptr as *mut DownloadService;
    let download_service = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    let url = config.url.to_string();
    let path = config.path.to_string();
    let config = DownloadConfiguration::new()
        .set_url(&url)
        .set_file_path(&path)
        .set_chunk_download(config.chunk_download)
        .set_chunk_size(config.chunk_size)
        .set_remote_version(config.version)
        .set_retry_times_on_failure(config.retry_times)
        .set_timeout(config.timeout)
        .build();
    let operation = download_service.add_downloader(config);
    Box::into_raw(Box::new(operation)) as *mut FFIDownloadOperation
}

#[no_mangle]
pub extern "C" fn get_download_status(ptr: *mut FFIDownloadOperation) -> u8 {
    let ptr = ptr as *mut DownloadOperation;
    let operation = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    operation.status().into()
}

#[no_mangle]
pub extern "C" fn get_download_is_done(ptr: *mut FFIDownloadOperation) -> bool {
    let ptr = ptr as *mut DownloadOperation;
    let operation = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    operation.is_done()
}

#[no_mangle]
pub extern "C" fn get_download_is_error(ptr: *mut FFIDownloadOperation) -> bool {
    let ptr = ptr as *mut DownloadOperation;
    let operation = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    operation.is_error()
}

#[no_mangle]
pub extern "C" fn get_download_error(ptr: *mut FFIDownloadOperation) -> *mut FFIString {
    let ptr = ptr as *mut DownloadOperation;
    let operation = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    let error = operation.error().to_string().into();
    Box::into_raw(Box::new(error))
}

#[no_mangle]
pub extern "C" fn free_download_error(ptr: *mut FFIString) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn get_downloaded_size(ptr: *mut FFIDownloadOperation) -> u64 {
    let ptr = ptr as *mut DownloadOperation;
    let operation = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    operation.downloaded_size()
}

#[no_mangle]
pub extern "C" fn get_download_progress(ptr: *mut FFIDownloadOperation) -> f64 {
    let ptr = ptr as *mut DownloadOperation;
    let operation = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    operation.progress()
}

#[no_mangle]
pub extern "C" fn stop_downloader(ptr: *mut FFIDownloadOperation) {
    let ptr = ptr as *mut DownloadOperation;
    let operation = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    operation.stop()
}

#[no_mangle]
pub extern "C" fn downloader_dispose(ptr: *mut FFIDownloadOperation) {
    let ptr = ptr as *mut DownloadOperation;
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}
