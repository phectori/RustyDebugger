use serde::Serialize;
use crc_all::Crc;

/// STX Start byte for every packet
pub const STX: u8 = 0x55;
/// ETX Stop byte for every packet
pub const ETX: u8 = 0xAA;

pub const COMMAND_GET_INFO: u8 = 0x49; // 'I'
pub const COMMAND_GET_VERSION: u8 = 0x56; // 'V'
pub const COMMAND_WRITE_REGISTER: u8 = 0x57; // 'W'

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Packet<T: Serialize> {
    stx: u8,
    content: T,
    crc: u8,
    etx: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Content<T: Serialize> {
    uc: u8,
    id: u8,
    p: T,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Generic {
    // Used for packages that only send a command
    // Such as: GetInfo, GetVersion
    pub command: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GetVersionResponse {
    pub command: u8,
    /// Debug library/protocol version
    pub dv3: u8,
    pub dv2: u8,
    pub dv01: u16,
    /// Application version
    pub av3: u8,
    pub av2: u8,
    pub av01: u16,
    /// Name
    pub name: String,
    /// Serial number
    pub sn: Vec<u8>,
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
            command: COMMAND_GET_INFO,
        }
    }

    pub fn get_version() -> Generic {
        Generic {
            command: COMMAND_GET_VERSION,
        }
    }

    pub fn write_register(off: u32, ctrl: u8, d: Vec<u8>) -> WriteRegister {
        WriteRegister {
            command: COMMAND_WRITE_REGISTER,
            off: off,
            ctrl: ctrl,
            d: d,
        }
    }

    pub fn serialize<T: Serialize>(packet: T) -> Vec<u8> {
        let c = Content {
            uc: 1, // TODO
            id: 1, // TODO
            p: packet,
        };

        let c_encoded = &bincode::serialize(&c).unwrap();

        // TODO: Move the crc algo declaration
        let mut crc8_maxim = Crc::<u8>::new(0x31, 8, 0, 0, true);
        let crc = crc8_maxim.update(&c_encoded);

        let t = Packet {
            stx: STX,
            content: c,
            crc: crc,
            etx: ETX,
        };
        bincode::config().little_endian().serialize(&t).unwrap()
    }

    pub fn deserialize<'a, T>(data: &'a Vec<u8>) -> T
    where
        T: serde::de::Deserialize<'a>,
    {
        bincode::deserialize(&data).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_info() {
        assert_eq!(
            PacketGenerator::serialize(PacketGenerator::get_info()),
            vec![STX, 0x01, 0x01, 0x49, 0xB5, ETX]
        );
    }

    #[test]
    fn get_version() {
        assert_eq!(
            PacketGenerator::serialize(PacketGenerator::get_version()),
            vec![STX, 0x01, 0x01, 0x56, 0x69, ETX]
        );
    }

    #[test]
    fn get_version_response() {
        assert_eq!(
            PacketGenerator::serialize(GetVersionResponse {
                command: COMMAND_GET_VERSION,
                dv3: 2,
                dv2: 3,
                dv01: 1113,
                av3: 2,
                av2: 3,
                av01: 1113,
                name: "Test".to_string(),
                sn: vec![1, 2, 3, 4],
            }),
            vec![
                STX,
                0x01,
                0x01,
                COMMAND_GET_VERSION,
                0x02,
                0x03,
                0x59,
                0x04,
                0x02,
                0x03,
                0x59,
                0x04,
                0x04,
                84,
                101,
                115,
                116,
                0x04,
                1,
                2,
                3,
                4,
                0x7F,
                ETX
            ]
        );
    }

    #[test]
    fn write_register() {
        let p = PacketGenerator::write_register(10, 0xF0, vec![1, 2, 3, 4]);

        assert_eq!(
            PacketGenerator::serialize(p),
            vec![STX, 0x01, 0x01, 0x57, 10, 0, 0, 0, 0xF0, 4, 1, 2, 3, 4, 0xB6, ETX]
        );
    }

    #[test]
    fn get_info_serialized() {
        let p = PacketGenerator::get_info();
        let serialized = PacketGenerator::serialize(&p);
        let deserialize: Packet<Content<Generic>> = PacketGenerator::deserialize(&serialized);

        assert_eq!(p, deserialize.content.p);
    }

    #[test]
    fn write_register_serialized() {
        let p = PacketGenerator::write_register(10, 0xF0, vec![1, 2, 3, 4]);
        let serialized = PacketGenerator::serialize(&p);
        let deserialize: Packet<Content<WriteRegister>> = PacketGenerator::deserialize(&serialized);

        assert_eq!(p, deserialize.content.p);
    }
}
