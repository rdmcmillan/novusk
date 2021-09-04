use super::kernel::*;
use super::modules::x86_modules_init;
use crate::drivers::drivers::drivers_init;

pub unsafe fn x86_kernel_init() {
    drivers_init();

    x86_modules_init();
    kinfo!("x86 modules initialized");
    x86_printk!("    ex1 module intialized");
}
