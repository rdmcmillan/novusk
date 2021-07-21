#![no_std]
#![no_main]

pub use novusk;

#[cfg(target_arch = "aarch64")]
pub use novusk::aarch64;

#[cfg(target_arch = "arm")]
pub use novusk::arm;

#[cfg(target_arch = "x86_64")]
pub use novusk::x86_64;

#[cfg(target_arch = "x86")]
pub use novusk::x86;

use core::panic::PanicInfo;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    loop {  }
}
