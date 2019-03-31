mod implementation;

use std::os::raw::c_char;
extern "C" {
    fn main_cpp();
}

use webfinger::resolve;

fn main() {
    use std::ffi::CString;
    let mut args = ::std::env::args();
    let app = CString::new(args.next().unwrap()).unwrap();
    unsafe {
        let res = resolve("acct:jalcine@playvicious.social", true)
            .expect("Error while fetching resource");

        println!("Places to get more informations about {}:", res.subject);
        for link in res.links.into_iter() {
            match link.href {
                Some(value) => println!("- {}", value),
                None => println!("nuffin"),
            }
        }
        main_cpp();
    }
}

pub mod interface {
    include!(concat!(env!("OUT_DIR"), "/src/interface.rs"));
}
