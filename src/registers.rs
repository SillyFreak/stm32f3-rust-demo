#![macro_use]

#[macro_export]
macro_rules! registers {
    (
        $($(#[$Reg_attr:meta])* const $Reg:ident: $T:ty = $value:expr,)+
    ) => {
        $($(#[$Reg_attr])* pub const $Reg: *mut $T = $value as *mut $T;)+
    };
}

