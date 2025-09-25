use std::ffi::CStr;
use std::os::raw::c_char;

/// Exposed C function for image conversion.
/// Returns 0 on success, nonzero on error.
#[no_mangle]
pub extern "C" fn convert_image_c(input: *const c_char, output: *const c_char) -> i32 {
    // Convert C strings to Rust &str
    let c_input = unsafe { CStr::from_ptr(input) };
    let c_output = unsafe { CStr::from_ptr(output) };

    let input_str = match c_input.to_str() {
        Ok(s) => s,
        Err(_) => return 1,
    };
    let output_str = match c_output.to_str() {
        Ok(s) => s,
        Err(_) => return 2,
    };

    let result = imgconv_core::convert_image(input_str.as_ref(), output_str.as_ref());
    if result.is_ok() { 0 } else { 3 }
}

// Alternative implementation (suggested by Copilot, based on convert_image taking string arguments rather than Path):
// #[no_mangle]
// pub extern "C" fn convert_image_c(input: *const c_char, output: *const c_char) -> i32 {
//     let input = unsafe { CStr::from_ptr(input).to_str() };
//     let output = unsafe { CStr::from_ptr(output).to_str() };
// 
//     match (input, output) {
//         (Ok(input), Ok(output)) => match convert_image(input, output) {
//             Ok(_) => 0,
//             Err(_) => 1,
//         },
//         _ => 1,
//     }
// }
