#![no_std]
#![no_main]

#[macro_use] extern crate novusk;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() {
    printk::printk!("\nKernel Main");
}
