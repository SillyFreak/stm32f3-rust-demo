use registers::{Reg, RegPtr};

bitflags! {
    flags Mode: u32 {
        const MODE_IN    = 0x00,
        const MODE_OUT   = 0x01,
        const MODE_AF    = 0x02,
        const MODE_AN    = 0x03,
    }
}

bitflags! {
    flags OType: u16 {
        const OTYPE_PP   = 0x00,
        const OTYPE_OD   = 0x01,
    }
}

bitflags! {
    flags OSpeed: u32 {
        const OSPEED_2MHZ  = 0x01,
        const OSPEED_10MHZ = 0x02,
        const OSPEED_50MHZ = 0x03,
    }
}

bitflags! {
    flags PuPd: u32 {
        const PUPD_NOPULL = 0x00,
        const PUPD_UP     = 0x01,
        const PUPD_DOWN   = 0x02,
    }
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct Port {
    pub MODER:     u32,      //0x00
    pub OTYPER:    u16,      //0x04
    pub RESERVED0: u16,      //0x06
    pub OSPEEDR:   u32,      //0x08
    pub PUPDR:     u32,      //0x0C
    pub IDR:       u16,      //0x10
    pub RESERVED1: u16,      //0x12
    pub ODR:       u16,      //0x14
    pub RESERVED2: u16,      //0x16
    pub BSRR:      u32,      //0x18
    pub LCKR:      u32,      //0x1C
    pub AFR:       [u32; 2], //0x20-0x24
    pub BRR:       u16,      //0x28
    pub RESERVED3: u16,      //0x2A
}

impl Port {
    pub fn init(&mut self, pins: Pin,
                mode: Mode, ospeed: OSpeed, otype: OType, pupd: PuPd) {
        let pins = pins.bits();
        for pin in 0..16 {
            if pins | (1 << pin) != 0 {
                self.OSPEEDR.shift_mask_set(ospeed.bits(), OSpeed::all().bits(), pin * 2);
                self.OTYPER.shift_mask_set(otype.bits(), OType::all().bits(), pin);
                self.MODER.shift_mask_set(mode.bits(), Mode::all().bits(), pin * 2);
                self.PUPDR.shift_mask_set(pupd.bits(), PuPd::all().bits(), pin * 2);
            }
        }
    }
}

registers! {
    const PORT_A: Port = 0x48000000,
    const PORT_B: Port = 0x48000400,
    const PORT_C: Port = 0x48000800,
    const PORT_D: Port = 0x48000C00,
    const PORT_E: Port = 0x48001000,
    const PORT_F: Port = 0x48001400,
}

bitflags! {
    flags Pin: u16 {
        const PIN0  = 1 <<  0,
        const PIN1  = 1 <<  1,
        const PIN2  = 1 <<  2,
        const PIN3  = 1 <<  3,
        const PIN4  = 1 <<  4,
        const PIN5  = 1 <<  5,
        const PIN6  = 1 <<  6,
        const PIN7  = 1 <<  7,
        const PIN8  = 1 <<  8,
        const PIN9  = 1 <<  9,
        const PIN10 = 1 << 10,
        const PIN11 = 1 << 11,
        const PIN12 = 1 << 12,
        const PIN13 = 1 << 13,
        const PIN14 = 1 << 14,
        const PIN15 = 1 << 15,
    }
}

