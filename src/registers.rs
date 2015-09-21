#![macro_use]
#![cfg_attr(test, feature(hash))]

#[macro_export]
macro_rules! registers {
    (registers: $Register:ident {
        $($(#[$Reg_attr:meta])* const $Reg:ident = $value:expr,)+
    }) => {
        $($(#[$Reg_attr])* pub const $Reg: *mut $Register = $value as *mut $Register;)+
    };
}

