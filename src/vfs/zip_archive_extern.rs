use std::ffi::{c_char, CStr, CString};
use std::io::Read;
use vfs_rs::zip::zip_archive::ZipArchive;
use crate::byte_buffer::ByteBuffer;
use crate::vfs::zip_file_extern::ZipFilePtr;

#[repr(C)]
pub struct ZipArchivePtr {}

#[no_mangle]
pub extern "C" fn ZipArchive_New(archive_path: *const c_char) -> *mut ZipArchivePtr {
    let archive_path = unsafe { CStr::from_ptr(archive_path).to_str().unwrap() };
    let zip_archive = ZipArchive::new(archive_path).unwrap();
    Box::into_raw(Box::new(zip_archive)) as *mut ZipArchivePtr
}

#[no_mangle]
pub extern "C" fn ZipArchive_FileExist(ptr: *mut ZipArchivePtr, file_path: *const c_char) -> bool {
    let ptr = ptr as *mut ZipArchive;
    let zip_archive = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    let file_path = unsafe { CStr::from_ptr(file_path).to_str().unwrap() };
    zip_archive.file_exist(file_path)
}

#[no_mangle]
pub extern "C" fn ZipArchive_Open(ptr: *mut ZipArchivePtr, file_path: *const c_char) -> *mut ZipFilePtr {
    let ptr = ptr as *mut ZipArchive;
    let zip_archive = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    let file_path = unsafe { CStr::from_ptr(file_path).to_str().unwrap() };
    let zip_file = zip_archive.by_name(file_path).unwrap();
    Box::into_raw(Box::new(zip_file)) as *mut ZipFilePtr
}

#[no_mangle]
pub extern "C" fn ZipArchive_ReadAllBytes(ptr: *mut ZipArchivePtr, file_path: *const c_char) -> *mut ByteBuffer {
    let ptr = ptr as *mut ZipArchive;
    let zip_archive = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    let file_path = unsafe { CStr::from_ptr(file_path).to_str().unwrap() };
    let mut zip_file = zip_archive.by_name(file_path).unwrap();
    let length = zip_file.len();
    let mut buffer = vec![0u8; length as usize];
    zip_file.read_exact(&mut buffer[0..length as usize]).unwrap();
    let byte_buffer = ByteBuffer::from_vec(buffer);
    return Box::into_raw(Box::new(byte_buffer));
}

#[no_mangle]
pub extern "C" fn ZipArchive_Dispose(ptr: *mut ZipArchivePtr) {
    let ptr = ptr as *mut ZipArchivePtr;
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}