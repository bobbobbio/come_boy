// copyright 2023 Remi Bernotavicius

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};

pub struct Mutex<T: ?Sized> {
    inner: UnsafeCell<crate::picosystem::pico_mutex>,
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

#[must_use]
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a Mutex<T>,
}

impl<'mutex, T: ?Sized> MutexGuard<'mutex, T> {
    unsafe fn new(lock: &'mutex Mutex<T>) -> Option<MutexGuard<'mutex, T>> {
        Some(MutexGuard { lock })
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { crate::picosystem::pico_mutex_exit(self.lock.inner.get()) }
    }
}

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}

impl<T> Mutex<T> {
    #[inline]
    pub fn new(t: T) -> Mutex<T> {
        let mut inner = MaybeUninit::uninit();
        unsafe { crate::picosystem::pico_mutex_init(inner.as_mut_ptr()) };

        Mutex {
            inner: UnsafeCell::new(unsafe { inner.assume_init() }),
            data: UnsafeCell::new(t),
        }
    }
}

impl<T: ?Sized> Mutex<T> {
    pub fn lock(&self) -> Option<MutexGuard<'_, T>> {
        unsafe {
            crate::picosystem::pico_mutex_enter_blocking(self.inner.get());
            MutexGuard::new(self)
        }
    }
}
