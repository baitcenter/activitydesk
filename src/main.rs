mod qtimpl;

extern "C" {
    fn main_cpp();
}

fn main() {
    unsafe {
        main_cpp();
    }
}

pub mod interface {
    include!(concat!(env!("OUT_DIR"), "/src/interface.rs"));
}
