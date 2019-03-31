mod implementation;

use std::os::raw::c_char;
extern "C" {
    fn main_cpp();
}

fn main() {
    use std::ffi::CString;
    let mut args = ::std::env::args();
    let app = CString::new(args.next().unwrap()).unwrap();
    unsafe {
        main_cpp();
    }
}

pub mod interface {
    include!(concat!(env!("OUT_DIR"), "/src/interface.rs"));
}
