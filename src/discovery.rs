extern crate core;

extern {
    fn SystemCoreClockUpdate();
    fn _delay_ms(delay: u32);
}

pub fn core_clock_update() {
    unsafe {
        SystemCoreClockUpdate();
    }
}

pub fn delay_ms(delay: u32) {
    unsafe {
        _delay_ms(delay);
    }
}

pub mod systick {
    extern {
        fn SysTick_Config_wrapper(ticks: u32) -> u32;
    }

    pub fn config(ticks: u32) -> u32 {
        unsafe {
            return SysTick_Config_wrapper(ticks);
        }
    }
}

pub mod led {
    extern {
        fn STM_EVAL_LEDInit(led: u8);
        fn STM_EVAL_LEDToggle(led: u8);
    }

    pub fn init(led: u8) {
        unsafe {
            STM_EVAL_LEDInit(led);
        }
    }

    pub fn toggle(led: u8) {
        unsafe {
            STM_EVAL_LEDToggle(led);
        }
    }
}

