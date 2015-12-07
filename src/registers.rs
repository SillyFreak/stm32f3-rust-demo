#![macro_use]

use core::ops::{Deref, DerefMut};

pub struct RegPtr<T> {
    pub ptr: *mut T,
}

impl<T> Deref for RegPtr<T> {
    type Target = T;

    fn deref(&self) -> &T {
        return unsafe { & *self.ptr };
    }
}

impl<T> DerefMut for RegPtr<T> {
    fn deref_mut(&mut self) -> &mut T {
        return unsafe { &mut *self.ptr };
    }
}

#[macro_export]
macro_rules! registers {
    (
        $($(#[$Reg_attr:meta])* const $Reg:ident: $T:ty = $value:expr,)+
    ) => {
        $($(#[$Reg_attr])* pub const $Reg: RegPtr<$T> = RegPtr {
            ptr: $value as *mut $T,
        };)+
    };
}

