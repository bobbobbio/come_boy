// copyright 2021 Remi Bernotavicius

use crate::picosystem;
use alloc::format;
use alloc::vec;
use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        picosystem::pen(219, 58, 4);
    }
    unsafe { picosystem::clear() };

    unsafe {
        picosystem::pen(255, 255, 255);
    }

    let msg = if let Some(loc) = info.location() {
        let file = loc.file().replace("/", "/ ");
        format!("panic occurred at {}:{}", file, loc.line())
    } else {
        "panic occurred from unknown place".into()
    };

    assert!(msg.is_ascii());

    let mut buffer = vec![0; msg.len() + 1];
    buffer[..(msg.len())].clone_from_slice(msg.as_bytes());

    unsafe { picosystem::text(buffer.as_ptr() as *const i8, 3, 3) }

    unsafe { crate::picosystem::wait_vsync() };
    unsafe { crate::picosystem::flip() };

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
