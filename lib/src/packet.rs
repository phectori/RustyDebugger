//use serde::de::Deserialize;
use serde::Serialize;

/// STX Start byte for every packet
pub const STX: u8 = 0x55;
/// ETX Stop byte for every packet
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
    /// Number of bytes to write is already serialized by bincode
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
            d: d,
        }
    }

    pub fn serialize<T: Serialize>(packet: T) -> Vec<u8> {
        let t = Packet {
            stx: STX,
            p: packet,
            etx: ETX,
        };
        bincode::config().little_endian().serialize(&t).unwrap()
    }

    // pub fn deserialize<'a, T> (data: Vec<u8>) -> bincode::Result<T> {
    //     bincode::deserialize(&data).unwrap()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_info() {
        assert_eq!(
            PacketGenerator::serialize(PacketGenerator::get_info()),
            vec![STX, 0x49, ETX]
        );
    }

    #[test]
    fn get_version() {
        assert_eq!(
            PacketGenerator::serialize(PacketGenerator::get_version()),
            vec![STX, 0x56, ETX]
        );
    }

    #[test]
    fn write_register() {
        let p = PacketGenerator::write_register(10, 0xF0, vec![1,2,3,4]);

        assert_eq!(
            PacketGenerator::serialize(p),
            vec![STX, 0x57, 10, 0, 0, 0, 0xF0, 4, 1,2,3,4, ETX]
        );
    }
}
