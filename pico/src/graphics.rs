// copyright 2021 Remi Bernotavicius

use crate::mutex::Mutex;
use crate::picosystem;
use alloc::boxed::Box;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;
use core::alloc::GlobalAlloc as _;
use core::cell::UnsafeCell;
use core::mem;
use core::slice;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

#[derive(Clone, Copy)]
pub struct Color(u16);

impl Color {
    #[inline(always)]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        const fn conv(v: u8) -> u16 {
            ((v as u32) * 100 / 255 * 0xF / 100) as u16
        }
        Self(conv(r) | conv(b) << 8 | conv(g) << 12)
    }
}

impl From<come_boy::rendering::Color> for Color {
    fn from(color: come_boy::rendering::Color) -> Self {
        Color::rgb(color.r, color.g, color.b)
    }
}

const SCREEN_BUFFER_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

#[repr(align(4))]
struct ScreenBuffer([u16; SCREEN_BUFFER_SIZE]);

impl ScreenBuffer {
    fn new_global(color: Color) -> &'static mut Self {
        let layout =
            core::alloc::Layout::from_size_align(mem::size_of::<ScreenBuffer>(), 4).unwrap();
        let ptr = unsafe { crate::ALLOCATOR.alloc(layout) } as *mut Self;
        assert!((ptr as u64).is_multiple_of(4));
        let s = unsafe { &mut *ptr };
        for i in 0..SCREEN_BUFFER_SIZE {
            s.0[i] = color.0;
        }
        s
    }
}

struct SharedScreenBufferPtr(Mutex<*mut ScreenBuffer>);

unsafe impl Sync for SharedScreenBufferPtr {}

impl SharedScreenBufferPtr {
    fn new(buffer: *mut ScreenBuffer) -> Self {
        Self(Mutex::new(buffer))
    }
}

struct BackBufferPtr(UnsafeCell<*mut SharedScreenBufferPtr>);

unsafe impl Sync for BackBufferPtr {}

impl BackBufferPtr {
    const fn new() -> Self {
        Self(UnsafeCell::new(core::ptr::null_mut()))
    }

    // safety: Must be called once while single threaded
    unsafe fn init(&self) {
        *unsafe { &mut *self.0.get() } = Box::into_raw(Box::new(SharedScreenBufferPtr::new(
            ScreenBuffer::new_global(Color::rgb(255, 0, 0)),
        )));
    }

    fn get(&self) -> &SharedScreenBufferPtr {
        let ptr = *(unsafe { &*self.0.get() });
        assert!(!ptr.is_null());
        unsafe { &*ptr }
    }
}

/// This pointer is shared between the two cores
static BACK: BackBufferPtr = BackBufferPtr::new();

/// Must be called only once while single-threaded
pub unsafe fn init_from_core0() {
    unsafe { picosystem::blend_copy() };

    BACK.init();
}

pub struct Graphics {
    writing: &'static mut ScreenBuffer,
    back: &'static SharedScreenBufferPtr,
}

#[allow(dead_code)]
impl Graphics {
    pub fn new() -> Self {
        Self {
            writing: ScreenBuffer::new_global(Color::rgb(0, 255, 0)),
            back: BACK.get(),
        }
    }
}

pub fn pen(r: u8, g: u8, b: u8) {
    unsafe { picosystem::pen(r, g, b) }
}

#[allow(dead_code)]
pub fn clear() {
    unsafe { picosystem::clear() }
}

pub fn text(msg: &str, x: i32, y: i32) {
    assert!(msg.is_ascii());

    let mut buffer = vec![0; msg.len() + 1];
    buffer[..(msg.len())].clone_from_slice(msg.as_bytes());

    unsafe { picosystem::text(buffer.as_ptr() as *const i8, x, y) }
}

pub fn filled_rect(x: i32, y: i32, w: i32, h: i32) {
    unsafe { picosystem::frect(x, y, w, h) }
}

impl come_boy::rendering::Renderer for Graphics {
    fn poll_events(&mut self) -> Vec<come_boy::rendering::Event> {
        vec![]
    }

    fn save_buffer(&self, _w: impl come_boy::io::Write) -> come_boy::io::Result<()> {
        unimplemented!()
    }

    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    fn color_pixel(&mut self, x: i32, y: i32, color: come_boy::rendering::Color) {
        let color: Color = color.into();
        if x < 0 || x >= SCREEN_WIDTH as i32 || y < 0 || y >= SCREEN_HEIGHT as i32 {
            return;
        }
        let color_data = self.writing as *mut ScreenBuffer as *mut u16;
        let color_data =
            unsafe { color_data.offset(SCREEN_WIDTH as isize * (y as isize) + x as isize) };
        unsafe { color_data.write(color.0) };
    }

    fn present(&mut self) {
        let mut back = self.back.0.lock().unwrap();
        let back_ptr = *back;
        *back = self.writing;
        self.writing = unsafe { &mut *back_ptr };
    }
}

/// Called periodically from core0
///
/// safety: The caller needs to make sure nothing is using the target buffer
pub unsafe fn draw_from_core0() {
    // Swap the shared buffer ptr with our local one.
    let target_buffer = unsafe { &mut *crate::picosystem::target_buffer() };
    let back = BACK.get().0.lock().unwrap();
    let src = slice::from_raw_parts(*back as *mut u16, SCREEN_BUFFER_SIZE);
    let dst = slice::from_raw_parts_mut(target_buffer.data as *mut u16, 240 * 240);

    for row in 0..SCREEN_HEIGHT {
        let row_start = (row * 240) + (240 - SCREEN_WIDTH) / 2;
        dst[row_start..(row_start + SCREEN_WIDTH)]
            .copy_from_slice(&src[(row * SCREEN_WIDTH)..((row + 1) * SCREEN_WIDTH)]);
    }

    // clear stats
    pen(0, 0, 0);
    filled_rect(3, 150, 240, 100);

    // draw stats
    pen(255, 0, 0);

    let perf_stats = unsafe { crate::emulator::PERF_STATS.get() }.lock().unwrap();
    let msg = format!("{}", &*perf_stats);
    text(&msg, 3, 150);
}
