use downloader_rs::download_configuration::DownloadConfiguration;
use crate::download::download_operation_extern::DownloadOperation;
use crate::ffi::FFIString;

#[repr(C)]
pub struct DownloadConfig {
    pub url: FFIString,
    pub path: FFIString,
    pub retry_times: u8,
    pub chunk_download: bool,
    pub version: i64,
    pub chunk_size: u64,
    pub timeout: u64,
}

#[repr(C)]
pub struct DownloadService {

}

#[no_mangle]
pub extern "C" fn DownloadService_Start(multi_thread: bool, thread_count: u16, parallel_count: u16) -> *mut DownloadService {
    let mut download_service = downloader_rs::download_service::DownloadService::new()
        .set_worker_thread_count(thread_count as usize)
        .set_multi_thread(multi_thread);
    download_service.set_parallel_count(parallel_count as usize);
    download_service.start_service();
    Box::into_raw(Box::new(download_service)) as *mut DownloadService
}

#[no_mangle]
pub extern "C" fn DownloadService_SetParallelCount(ptr: *mut DownloadService, count: u16) {
    let ptr = ptr as *mut downloader_rs::download_service::DownloadService;
    let service = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    service.set_parallel_count(count as usize)
}

#[no_mangle]
pub extern "C" fn DownloadService_Stop(ptr: *mut DownloadService) {
    let ptr = ptr as *mut downloader_rs::download_service::DownloadService;
    if ptr.is_null() {
        return;
    }
    unsafe {
        let download_service = Box::from_raw(ptr);
        download_service.stop();
    }
}

#[no_mangle]
pub extern "C" fn DownloadService_AddDownload(ptr: *mut DownloadService, config: DownloadConfig) -> *mut DownloadOperation {
    let ptr = ptr as *mut downloader_rs::download_service::DownloadService;
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
    Box::into_raw(Box::new(operation)) as *mut DownloadOperation
}
