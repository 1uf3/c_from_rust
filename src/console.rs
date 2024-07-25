use core::ffi::c_int;

use crate::{Devsw, Inode};



const CONSOLE: usize = 1;


#[no_mangle]
pub extern "C" fn read_func(_inode: *mut Inode, _buf: *mut i8, _n: c_int) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn consolewrite(_ip: *mut Inode, _buf: *mut i8, _n: c_int) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn consoleinit(devsw: *mut Devsw) {
    unsafe {
        (*devsw.add(CONSOLE)).read = Some(read_func);
        (*devsw.add(CONSOLE)).write = Some(consolewrite);
    }
}