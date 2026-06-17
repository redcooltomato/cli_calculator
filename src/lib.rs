use std::os::raw::c_char;
use std::ffi::{CStr, CString};

use crate::calculator::{calculate_expression};
mod calculator;
mod operators;

static mut STRING_POINTER: *mut c_char = 0 as *mut c_char;

fn store_string_on_heap(string_to_store: &str) -> *mut c_char {
    let pntr = CString::new(string_to_store).unwrap().into_raw();
    unsafe {
        STRING_POINTER = pntr;
    }
    return pntr;
}

// this stuff is for c# (didnt test c++ or c) to call rust code, returns errors sometimes
//
// !!
// remember to call free_string() after each use
// !!

#[unsafe(no_mangle)]
pub extern "C" fn free_string() {
    unsafe {
        let _ = CString::from_raw(STRING_POINTER);
        STRING_POINTER = 0 as *mut c_char;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn calc_string(txt_ptr: *const c_char) -> *const c_char {
    let response: String = 'block: {
        unsafe {
            let txt = CStr::from_ptr(txt_ptr).to_str();
            if txt.is_err() {
                break 'block txt.unwrap_err().to_string();
            }
            let txt: String = txt.unwrap().to_string();

            let result = calculate_expression(&txt);
            if result.is_err() {
                break 'block result.unwrap_err().to_string();
            }
            result.unwrap().to_string()
        }
    };

    store_string_on_heap(&response) as *const c_char
}