use crate::ffi::FFIString;

#[repr(C)]
pub struct DownloadOperation {}

#[no_mangle]
pub extern "C" fn DownloadOperation_GetDownloadStatus(ptr: *mut DownloadOperation) -> u8 {
    let ptr = ptr as *mut downloader_rs::download_operation::DownloadOperation;
    let operation = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    operation.status().into()
}

#[no_mangle]
pub extern "C" fn DownloadOperation_IsDone(ptr: *mut DownloadOperation) -> bool {
    let ptr = ptr as *mut downloader_rs::download_operation::DownloadOperation;
    let operation = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    operation.is_done()
}

#[no_mangle]
pub extern "C" fn DownloadOperation_IsError(ptr: *mut DownloadOperation) -> bool {
    let ptr = ptr as *mut downloader_rs::download_operation::DownloadOperation;
    let operation = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    operation.is_error()
}

#[no_mangle]
pub extern "C" fn DownloadOperation_Error(ptr: *mut DownloadOperation) -> *mut FFIString {
    let ptr = ptr as *mut downloader_rs::download_operation::DownloadOperation;
    let operation = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    let error = operation.error().to_string().into();
    Box::into_raw(Box::new(error))
}

#[no_mangle]
pub extern "C" fn DownloadOperation_GetDownloadedSize(ptr: *mut DownloadOperation) -> u64 {
    let ptr = ptr as *mut downloader_rs::download_operation::DownloadOperation;
    let operation = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    operation.downloaded_size()
}

#[no_mangle]
pub extern "C" fn DownloadOperation_GetDownloadProgress(ptr: *mut DownloadOperation) -> f64 {
    let ptr = ptr as *mut downloader_rs::download_operation::DownloadOperation;
    let operation = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    operation.progress()
}

#[no_mangle]
pub extern "C" fn DownloadOperation_Stop(ptr: *mut DownloadOperation) {
    let ptr = ptr as *mut downloader_rs::download_operation::DownloadOperation;
    let operation = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    operation.stop()
}

#[no_mangle]
pub extern "C" fn DownloadOperation_Dispose(ptr: *mut DownloadOperation) {
    let ptr = ptr as *mut downloader_rs::download_operation::DownloadOperation;
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}