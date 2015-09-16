#![feature(no_std)]
#![feature(core)]
#![feature(lang_items)]
#![no_std]

#![crate_type="staticlib"]

mod runtime;
mod discovery;

#[no_mangle]
pub fn main() {
    discovery::systick::config(72);

	discovery::core_clock_update();

    discovery::led::init(0);
    loop {
        discovery::led::toggle(0);
		discovery::delay_ms(500);
    }
}

//TODO stubs that the linker otherwise misses; seems to have to do with assert

#[no_mangle]
pub fn _exit() -> ! {
    loop {}
}

#[no_mangle]
pub fn _kill() -> ! {
    loop {}
}

#[no_mangle]
pub fn _getpid() -> ! {
    loop {}
}
