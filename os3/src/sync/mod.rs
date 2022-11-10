use core::cell::{RefCell, RefMut};

pub struct UPSafeCell<T> {
    inner: RefCell<T>
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    /// 安全边界: 用户需要保证 value 的 Sync 特性, 在单个线程或者处理器中使用
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value)
        }
    }

    /// 安全边界: 用户需要保证 value 的 Sync 特性, 在单个线程或者处理器中使用
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}