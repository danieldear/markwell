use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use markdown_core::parse_markdown;
use markdown_render_html::render_html;

/// Render markdown input to HTML and return an owned C string.
///
/// Returns null pointer on invalid input or memory errors.
///
/// # Safety
/// `input` must be a valid, null-terminated C string pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn markdown_render_html(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return std::ptr::null_mut();
    }

    let markdown = {
        // SAFETY: caller provides a valid null-terminated C string.
        let c_str = unsafe { CStr::from_ptr(input) };
        match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        }
    };

    let doc = match parse_markdown(markdown) {
        Ok(doc) => doc,
        Err(_) => return std::ptr::null_mut(),
    };

    match CString::new(render_html(&doc)) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Free a string returned by `markdown_render_html`.
///
/// # Safety
/// `ptr` must have been returned by `markdown_render_html` and not already freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn markdown_string_free(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    // SAFETY: pointer ownership is transferred back from FFI caller.
    let _ = unsafe { CString::from_raw(ptr) };
}
