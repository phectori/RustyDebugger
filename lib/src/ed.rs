use serde::Serialize;

/// STX/ETX byte
pub const STX: u8 = 0x55;
pub const ETX: u8 = 0xAA;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Packet<T: Serialize> {
    stx: u8,
    p: T,
    etx: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Generic {
    pub cmd: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WriteRegister {
    pub command: u8,
    /// Offset address in bytes (4Gb addressable)
    pub off: u32,
    /// Control
    pub ctrl: u8,
    /// Number of bytes to write
    pub size: u8,
    /// Data-bytes, only number of bytes used are sent
    pub d: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WriteRegisterResponse {
    pub command: u8,
    /// Result:
    /// 0x00 = ok, value is written
    /// 0x01 = invalid (offset) address
    /// 0x02 = error dereferencing (null-pointer appeared at some dereference)
    pub result: u8,
}

pub struct PacketGenerator {}

impl PacketGenerator {
    pub fn get_info() -> Generic {
        Generic {
            // 'I'
            cmd: 0x49,
        }
    }

    pub fn get_version() -> Generic {
        Generic {
            // 'V'
            cmd: 0x56,
        }
    }

    pub fn write_register(off: u32, ctrl: u8, d: Vec<u8>) -> WriteRegister {
        WriteRegister {
            // 'W'
            command: 0x57,
            off: off,
            ctrl: ctrl,
            size: d.len() as u8,
            d: d,
        }
    }

    pub fn serialize<T: Serialize>(packet: T) -> Vec<u8> {
        let t = Packet {
            stx: STX,
            p: packet,
            etx: ETX
        };
        bincode::serialize(&t).unwrap()
    }
}
