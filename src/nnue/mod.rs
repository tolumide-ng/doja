// use std::{ffi::{self, CString}, io::{stdout, Write}, os::raw::c_char};

// use binread::BinRead;


// #[derive(Debug, Clone, BinRead)]
// pub struct NNUE;

// extern "C" {
//     fn load_eval_file(eval_file: *const c_char) -> bool;
// }

// impl NNUE {
//     // #[no_mangle]
//     // pub(crate) fn new(eval_file: *const c_char) -> Result<NNUE, &'static str> {
//     //     let eval_file_cstr = unsafe { ffi::CStr::from_ptr(eval_file) };
//     //     let eval_file_str = eval_file_cstr.to_string_lossy();
//     //     write!(stdout(), "Loading NNUE: {eval_file_str} \n").unwrap();
//     //     stdout().flush().unwrap();

//     //     let load_success = unsafe { load_eval_file(eval_file) };

//     //     if load_success {
//     //         let loaded_file = eval_file_cstr.to_owned();
//     //         let nnue = Self(loaded_file);
//     //         stdout().flush().unwrap();
//     //         return Ok(nnue);
//     //     } else {
//     //         stdout().flush().unwrap();
//     //         return Err("NNUE file not found");
//     //     }
//     // }
// }

use binread::BinRead;
use lazy_static::lazy_static;

#[derive(Debug, Clone, BinRead)]
pub struct NNUE {}

lazy_static! {
    pub static ref STOCK: NNUE = {
        let mut reader = std::io::Cursor::new(include_bytes!("./nn-56a5f1c4173a.nnue"));
        NNUE::read(&mut reader).unwrap()
    };
    // pub static ref NAME: String
}