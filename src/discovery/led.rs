use registers::RegPtr;
use stm32f3::gpio;
use stm32f3::rcc;

pub struct Led {
    pub led:  u8,
    pub port: RegPtr<gpio::Port>,
    pub pin:  gpio::Pin,
    pub clk:  rcc::AHBENR,
}

extern {
    fn STM_EVAL_LEDInit(led: u8);
}

impl Led {
    pub fn init(&mut self) {
        unsafe {
            STM_EVAL_LEDInit(self.led);
        }
    }

    pub fn toggle(&mut self) {
        let port = &mut *self.port;
        let pin = self.pin;

        port.ODR ^= pin.bits() as u16;
    }
}

pub const LED6:  Led = Led { led: 3, port: gpio::PORT_E, pin: gpio::PIN15, clk: rcc::GPIOEEN, };
pub const LED8:  Led = Led { led: 5, port: gpio::PORT_E, pin: gpio::PIN14, clk: rcc::GPIOEEN, };
pub const LED10: Led = Led { led: 7, port: gpio::PORT_E, pin: gpio::PIN13, clk: rcc::GPIOEEN, };
pub const LED9:  Led = Led { led: 6, port: gpio::PORT_E, pin: gpio::PIN12, clk: rcc::GPIOEEN, };
pub const LED7:  Led = Led { led: 4, port: gpio::PORT_E, pin: gpio::PIN11, clk: rcc::GPIOEEN, };
pub const LED5:  Led = Led { led: 2, port: gpio::PORT_E, pin: gpio::PIN10, clk: rcc::GPIOEEN, };
pub const LED3:  Led = Led { led: 0, port: gpio::PORT_E, pin: gpio::PIN9,  clk: rcc::GPIOEEN, };
pub const LED4:  Led = Led { led: 1, port: gpio::PORT_E, pin: gpio::PIN8,  clk: rcc::GPIOEEN, };

pub const LED: [Led; 8] = [
    LED3, LED4, LED5, LED6,
    LED7, LED8, LED9, LED10,
];

