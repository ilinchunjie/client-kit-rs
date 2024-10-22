use std::ffi::CStr;
use downloader_rs::download_configuration::DownloadConfiguration;
use downloader_rs::download_service::DownloadService;
use crate::download::download_config_extern::DownloadConfig;
use crate::download::download_operation_extern::DownloadOperationPtr;
#[repr(C)]
pub struct DownloadServicePtr {

}

#[no_mangle]
pub extern "C" fn DownloadService_Start(multi_thread: bool, thread_count: u16, parallel_count: u16) -> *mut DownloadServicePtr {
    let mut download_service = DownloadService::new()
        .set_worker_thread_count(thread_count as usize)
        .set_multi_thread(multi_thread);
    download_service.set_parallel_count(parallel_count as usize);
    download_service.start_service();
    Box::into_raw(Box::new(download_service)) as *mut DownloadServicePtr
}

#[no_mangle]
pub extern "C" fn DownloadService_SetParallelCount(ptr: *mut DownloadServicePtr, count: u16) {
    let ptr = ptr as *mut DownloadService;
    let service = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    service.set_parallel_count(count as usize)
}

#[no_mangle]
pub extern "C" fn DownloadService_Stop(ptr: *mut DownloadServicePtr) {
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
pub extern "C" fn DownloadService_AddDownload(ptr: *mut DownloadServicePtr, config: DownloadConfig) -> *mut DownloadOperationPtr {
    let ptr = ptr as *mut DownloadService;
    let download_service = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    let url = unsafe { CStr::from_ptr(config.url).to_str().unwrap() };
    let path = unsafe { CStr::from_ptr(config.path).to_str().unwrap() };
    let config = DownloadConfiguration::new()
        .set_url(url)
        .set_file_path(path)
        .set_chunk_download(config.chunk_download)
        .set_chunk_size(config.chunk_size)
        .set_remote_version(config.version)
        .set_retry_times_on_failure(config.retry_times)
        .set_timeout(config.timeout)
        .set_download_in_memory(config.download_in_memory)
        .build();
    let operation = download_service.add_downloader(config);
    Box::into_raw(Box::new(operation)) as *mut DownloadOperationPtr
}
