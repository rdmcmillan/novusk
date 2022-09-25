use core::borrow::Borrow;
use novuskinc::platform::{early_device_init, DEVICE_INIT_ERRORS};
use novuskinc::serial::early_serial_init;
use kinfo::{InfoDisplay, status::KStatus};
use setup::{BootSetup, SetupReturn};

pub(crate) struct Aarch64Boot;

impl Aarch64Boot {
    pub fn new() -> Self {
        return Aarch64Boot;
    }

    pub fn setup(&self) {
        let linker_mem = unsafe { self.linker_setup() };
        let early_dev = self.early_device_init();
        let early_serial = self.early_serial_io_init();

        if linker_mem.0.is_err() {
            kinfo!(KStatus {
                status: "not ok",
                should_panic: true,
                panic_message: Some(linker_mem.1),
                main_message: "Linker memory setup failed",
                messages: None,
            });
        } else if early_dev.0.is_err() {
            kinfo!(KStatus {
                status: "not ok",
                should_panic: false,
                panic_message: None,
                main_message: "Early device initialization failed",
                messages: Some(&["This can cause problems latter"]),
            });
        } else if early_serial.0.is_err() {
            kinfo!(KStatus {
                status: "not ok",
                should_panic: false,
                panic_message: None,
                main_message: early_serial.0.err().unwrap(),
                messages: None,
            });
        }
    }
}

impl BootSetup for Aarch64Boot {
    fn early_device_init(&self) -> SetupReturn {
        let early_dev_init = unsafe { early_device_init() };

        if early_dev_init == 0 {
            return (Ok(()), "Early device drivers initialized");
        } else { return (Err("Early device initialization failed"), DEVICE_INIT_ERRORS[early_dev_init as usize]); }
    }

    fn early_serial_io_init(&self) -> SetupReturn {
        if unsafe { early_serial_init() } == 0 {
            return (Ok(()), "Early serial initialized");
        } else { return (Err("Early serial initialization failed"), "Failed to initialize early serial driver"); }
    }

    unsafe fn linker_setup(&self) -> SetupReturn {
        extern "C" {
            static mut __bss_start: u64;
            static mut __bss_end: u64;
        }

        r0::zero_bss(&mut __bss_start, &mut __bss_end);

        if __bss_start == 0 {
            return (Err("Linker mem setup failed"), "Failed to clear __bss_start");
        } else { return (Ok(()), "Successfully setup linker memory"); }
    }
}
