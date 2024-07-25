#![no_std]

mod console;

use core::ffi::c_int;

#[repr(C)]
pub struct Inode;

#[derive(Debug)]
#[repr(C)]
pub struct Devsw {
    read: Option<unsafe extern "C" fn(*mut Inode, *mut i8, c_int) -> c_int>,
    write: Option<unsafe extern "C" fn(*mut Inode, *mut i8, c_int) -> c_int>,
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic!("{}", info)
}
