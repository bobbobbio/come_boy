// copyright 2021 Remi Bernotavicius

use alloc::format;
use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::graphics::pen(219, 58, 4);
    crate::graphics::clear();
    crate::graphics::pen(255, 255, 255);

    let msg = if let Some(loc) = info.location() {
        let file = loc.file().replace("/", "/ ");
        format!("panic occurred at {}:{}", file, loc.line())
    } else {
        "panic occurred from unknown place".into()
    };

    crate::graphics::text(&msg, 3, 3);

    unsafe { crate::picosystem::wait_vsync() };
    unsafe { crate::picosystem::flip() };

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
