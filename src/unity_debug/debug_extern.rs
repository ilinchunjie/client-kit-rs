use std::ffi::{c_char, CString};

pub type UnityDebugFunction = unsafe extern "C" fn(content: *const c_char);

static mut UNITY_DEBUG_LOG: Option<UnityDebugFunction> = None;
static mut UNITY_DEBUG_WARNING: Option<UnityDebugFunction> = None;
static mut UNITY_DEBUG_ERROR: Option<UnityDebugFunction> = None;


#[no_mangle]
extern fn BindUnityEngineDebugLog(func: UnityDebugFunction) {
    unsafe {
        UNITY_DEBUG_LOG = Some(func);
    }
}

#[no_mangle]
extern fn BindUnityEngineDebugWarning(func: UnityDebugFunction) {
    unsafe {
        UNITY_DEBUG_WARNING = Some(func);
    }
}

#[no_mangle]
extern fn BindUnityEngineDebugError(func: UnityDebugFunction) {
    unsafe {
        UNITY_DEBUG_ERROR = Some(func);
    }
}

pub fn unity_log(content: &str) {
    let c_str = CString::new(content).unwrap();
    unsafe {
        match UNITY_DEBUG_LOG {
            Some(log) => {
                log(c_str.as_ptr())
            }
            None => {}
        }
    }
}

pub fn unity_warning(content: &str) {
    let c_str = CString::new(content).unwrap();
    unsafe {
        match UNITY_DEBUG_LOG {
            Some(warning) => {
                warning(c_str.as_ptr())
            }
            None => {}
        }
    }
}

pub fn unity_error(content: &str) {
    let c_str = CString::new(content).unwrap();
    unsafe {
        match UNITY_DEBUG_LOG {
            Some(error) => {
                error(c_str.as_ptr())
            }
            None => {}
        }
    }
}