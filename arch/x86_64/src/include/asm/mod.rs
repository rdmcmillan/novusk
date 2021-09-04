global_asm!(include_str!("x86.S"));

extern "C" {
    fn halt() -> !;
    fn clear_interrupts();
}

pub(crate) unsafe fn hlt() -> ! {
    halt()
}

pub(crate) unsafe fn cli() {
    clear_interrupts();
}
