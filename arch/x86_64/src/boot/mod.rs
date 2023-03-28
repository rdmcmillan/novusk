use core::arch::{asm};
use novuskinc::kernel::types::KernelType;
use crate::kernel::vga::writer::VgaWriter;

core::arch::global_asm!(include_str!("start.S"));

pub mod cpu;
pub mod main;
pub mod setup;

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {

}

#[no_mangle]
pub extern "C" fn _kernel_version() -> (u8, u8, u8) { (3, 0, 2) }

#[no_mangle]
pub extern "C" fn _kernel_type() -> KernelType {
    KernelType::RegularKernel {
        init_path: "",
        start_path: "",
        init_fun: kernel_init,
    }
}
