#![allow(dead_code)]

pub mod gpio {
    #[repr(C, packed)]
    #[allow(non_snake_case)]
    pub struct Port {
        pub MODER:     u32, //0x00
        pub OTYPER:    u16, //0x04
        pub RESERVED0: u16, //0x06
        pub OSPEEDR:   u32, //0x08
        pub PUPDR:     u32, //0x0C
        pub IDR:       u16, //0x10
        pub RESERVED1: u16, //0x12
        pub ODR:       u16, //0x14
        pub RESERVED2: u16, //0x16
        pub BSRR:      u32, //0x18
        pub LCKR:      u32, //0x1C
        pub AFR:       u32, //0x20-0x24
        pub BRR:       u16, //0x28
        pub RESERVED3: u16, //0x2A
    }

    pub const PORT_A: *mut Port = 0x48000000 as *mut Port;
    pub const PORT_B: *mut Port = 0x48000400 as *mut Port;
    pub const PORT_C: *mut Port = 0x48000800 as *mut Port;
    pub const PORT_D: *mut Port = 0x48000C00 as *mut Port;
    pub const PORT_E: *mut Port = 0x48001000 as *mut Port;
    pub const PORT_F: *mut Port = 0x48001400 as *mut Port;

    bitflags! {
        flags Pin: u32 {
            const PIN0  = (1 <<  0),
            const PIN1  = (1 <<  1),
            const PIN2  = (1 <<  2),
            const PIN3  = (1 <<  3),
            const PIN4  = (1 <<  4),
            const PIN5  = (1 <<  5),
            const PIN6  = (1 <<  6),
            const PIN7  = (1 <<  7),
            const PIN8  = (1 <<  8),
            const PIN9  = (1 <<  9),
            const PIN10 = (1 << 10),
            const PIN11 = (1 << 11),
            const PIN12 = (1 << 12),
            const PIN13 = (1 << 13),
            const PIN14 = (1 << 14),
            const PIN15 = (1 << 15),
        }
    }
}

