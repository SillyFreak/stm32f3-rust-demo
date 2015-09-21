extern crate core;

use stm32f3::gpio;

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

pub struct Led {
    pub led:  u8,
    pub port: *mut gpio::Port,
    pub pin:  gpio::Pin,
}

extern {
    fn STM_EVAL_LEDInit(led: u8);
}

impl Led {
    pub fn init(&self) {
        unsafe {
            STM_EVAL_LEDInit(self.led);
        }
    }

    pub fn toggle(&self) {
        unsafe {
            let port = &mut *self.port;
            let pin = self.pin;

            port.ODR ^= pin.bits() as u16;
        }
    }
}

pub const LED6:  Led = Led { led: 3, port: gpio::PORT_E, pin: gpio::PIN15, };
pub const LED8:  Led = Led { led: 5, port: gpio::PORT_E, pin: gpio::PIN14, };
pub const LED10: Led = Led { led: 7, port: gpio::PORT_E, pin: gpio::PIN13, };
pub const LED9:  Led = Led { led: 6, port: gpio::PORT_E, pin: gpio::PIN12, };
pub const LED7:  Led = Led { led: 4, port: gpio::PORT_E, pin: gpio::PIN11, };
pub const LED5:  Led = Led { led: 2, port: gpio::PORT_E, pin: gpio::PIN10, };
pub const LED3:  Led = Led { led: 0, port: gpio::PORT_E, pin: gpio::PIN9,  };
pub const LED4:  Led = Led { led: 1, port: gpio::PORT_E, pin: gpio::PIN8,  };

pub const LED: [Led; 8] = [
    LED3, LED4, LED5, LED6,
    LED7, LED8, LED9, LED10,
];
