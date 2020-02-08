/// STX/ETX byte
pub const STX: u8 = 0x55;
pub const ETX: u8 = 0xAA;

#[derive(Protocol, Debug, PartialEq)]
pub struct Generic {
    pub stx: u8,
    pub cmd: u8,
    pub etx: u8,
}

#[derive(Protocol, Debug, PartialEq)]
pub struct WriteRegister {
    pub stx: u8,
    pub command: u8,
    /// Offset address in bytes (4Gb addressable)
    pub off: u32,
    /// Control
    pub ctrl: u8,
    /// Number of bytes to write
    pub size: u8,
    /// Data-bytes, only number of bytes used are sent
    pub d: Vec<u8>,
    pub etx: u8,
}

#[derive(Protocol, Debug, PartialEq)]
pub struct WriteRegisterResponse {
    pub stx: u8,
    pub command: u8,
    /// Result:
    /// 0x00 = ok, value is written
    /// 0x01 = invalid (offset) address
    /// 0x02 = error dereferencing (null-pointer appeared at some dereference)
    pub result: u8,
    pub etx: u8,
}

pub struct PacketGenerator {}

impl PacketGenerator {
    pub fn get_info() -> Generic {
        Generic {
            stx: STX,
            // 'I'
            cmd: 0x49,
            etx: ETX,
        }
    }

    pub fn get_version() -> Generic {
        Generic {
            stx: STX,
            // 'V'
            cmd: 0x56,
            etx: ETX,
        }
    }

    pub fn write_register(off: u32, ctrl: u8, d: Vec<u8>) -> WriteRegister {
        WriteRegister {
            stx: STX,
            // 'W'
            command: 0x57,
            off: off,
            ctrl: ctrl,
            size: d.len() as u8,
            d: d,
            etx: ETX,
        }
    }
}

// #[derive(Protocol, Debug, PartialEq)]
// #[protocol(discriminant = "integer")]
// #[repr(u8)] // Force discriminators to be 8-bit.
// pub enum Packet {
//     #[protocol(discriminator(0x55))] // This is STX
//     Content(Content),
// }

// #[derive(Protocol, Debug, PartialEq)]
// #[protocol(discriminant = "integer")]
// #[repr(u8)] // Force discriminators to be 8-bit.
// pub enum Content {
//     #[protocol(discriminator(0x49))] // 'I'
//     GetInfo(u8),
//     #[protocol(discriminator(0x56))] // 'V'
//     GetVersion(u8),
//     #[protocol(discriminator(0x57))] // 'W'
//     WriteRegister(WriteRegister),
//     #[protocol(discriminator(0x51))] // 'Q'
//     QueryRegister(u8),
//     #[protocol(discriminator(0x43))] // 'C'
//     ConfigChannel(u8),
//     #[protocol(discriminator(0x44))] // 'D'
//     Decimation(u8),
//     #[protocol(discriminator(0x54))] // 'T'
//     ResetTime(u8),
//     #[protocol(discriminator(0x52))] // 'R'
//     ReadChannelData(u8),
//     #[protocol(discriminator(0x53))] // 'S'
//     DebugString(u8),
// }
