use std::io::{Read, Seek, SeekFrom};
use vfs_rs::zip::zip_file::ZipFile;
use crate::byte_buffer;
use crate::byte_buffer::ByteBuffer;

#[repr(u8)]
pub enum SeekOrigin {
    Begin = 0,
    Current = 1,
    End = 2,
}

#[repr(C)]
pub struct ZipFilePtr {}

#[no_mangle]
pub extern "C" fn ZipFile_Length(ptr: *mut ZipFilePtr) -> i64 {
    let ptr = ptr as *mut ZipFile;
    let zip_file = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    return zip_file.len() as i64;
}

#[no_mangle]
pub extern "C" fn ZipFile_Seek(ptr: *mut ZipFilePtr, offset: i64, seek_origin: SeekOrigin) -> i64 {
    let ptr = ptr as *mut ZipFile;
    let mut zip_file = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    match seek_origin {
        SeekOrigin::Begin => zip_file.seek(SeekFrom::Start(offset as u64)).unwrap() as i64,
        SeekOrigin::Current => zip_file.seek(SeekFrom::Current(offset)).unwrap() as i64,
        SeekOrigin::End => zip_file.seek(SeekFrom::End(offset)).unwrap() as i64
    }
}

#[no_mangle]
pub extern "C" fn ZipFile_Position(ptr: *mut ZipFilePtr, offset: i64, seek_origin: SeekOrigin) -> i64 {
    let ptr = ptr as *mut ZipFile;
    let mut zip_file = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    return zip_file.stream_position().unwrap() as i64;
}

#[no_mangle]
pub extern "C" fn ZipFile_Read(ptr: *mut ZipFilePtr, length: i32) -> *mut ByteBuffer {
    let ptr = ptr as *mut ZipFile;
    let mut zip_file = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    let mut bytes = vec![0u8; length as usize];
    let read_length = zip_file.read(&mut bytes[0..length as usize]).unwrap();
    let byte_buffer = ByteBuffer::from_slice(&bytes[0..read_length]);
    return Box::into_raw(Box::new(byte_buffer));
}

#[no_mangle]
pub extern "C" fn ZipFile_Dispose(ptr: *mut ZipFilePtr) {
    let ptr = ptr as *mut ZipFile;
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}