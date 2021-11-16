// copyright 2021 Remi Bernotavicius

use crate::graphics::Graphics;
use alloc::format;
use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let graphics = Graphics::new();

    graphics.set_pen(crate::graphics::Color::rgb(30, 12, 0));
    graphics.clear();

    graphics.set_pen(crate::graphics::Color::rgb(0, 0, 0));

    let message = if let Some(loc) = info.location() {
        let file = loc.file().replace("/", "/ ");
        format!("panic occurred at {}:{}", file, loc.line())
    } else {
        "panic occurred from unknown place".into()
    };

    graphics.text(&message);

    unsafe { crate::picosystem::wait_vsync() };
    unsafe { crate::picosystem::flip() };

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}