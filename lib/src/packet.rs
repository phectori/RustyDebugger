use crc_all::Crc;
use serde::Serialize;

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
    pub content: T,
    crc: u8,
    etx: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Content<T: Serialize> {
    pub uc: u8,
    pub id: u8,
    pub command: u8,
    pub p: T,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Generic {
    // Used for packages that only send a command
// Such as: GetInfo, GetVersion
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GetVersionResponse {
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
    /// Result:
    /// 0x00 = ok, value is written
    /// 0x01 = invalid (offset) address
    /// 0x02 = error dereferencing (null-pointer appeared at some dereference)
    pub result: u8,
}

pub struct PacketGenerator {
    uc : u8,
    id : u8,
    crc8_maxim: crc_all::Crc<u8>,
}

impl Default for PacketGenerator {
    fn default() -> Self {
        PacketGenerator {
            uc: 1,
            id: 1,
            crc8_maxim: Crc::<u8>::new(0x31, 8, 0, 0, true),
        }
    }
}

impl PacketGenerator {
    pub fn get_info(&mut self) -> Content<Generic> {
        Content {
            uc: self.uc,
            id: self.id,
            command: COMMAND_GET_INFO,
            p: Generic {},
        }
    }

    pub fn get_version(&mut self) -> Content<Generic> {
        Content {
            uc: self.uc,
            id: self.id,
            command: COMMAND_GET_VERSION,
            p: Generic {},
        }
    }

    pub fn get_version_response(&mut self) -> Content<GetVersionResponse> {
        Content {
            uc: self.uc,
            id: self.id,
            command: COMMAND_GET_VERSION,
            p: GetVersionResponse {
                dv3: 2,
                dv2: 3,
                dv01: 1113,
                av3: 2,
                av2: 3,
                av01: 1113,
                name: "Test".to_string(),
                sn: vec![1, 2, 3, 4],
            },
        }
    }

    pub fn write_register(&mut self, off: u32, ctrl: u8, d: Vec<u8>) -> Content<WriteRegister> {
        Content {
            uc: self.uc,
            id: self.id,
            command: COMMAND_WRITE_REGISTER,
            p: WriteRegister {
                off: off,
                ctrl: ctrl,
                d: d,
            },
        }
    }

    pub fn serialize<T: Serialize>(&mut self, packet: T) -> Vec<u8> {
        let mut serialized = bincode::serialize(&packet).unwrap();

        // Calculate crc
        let crc = self.crc8_maxim.update(&serialized);

        // Prepend STX
        serialized.splice(0..0, vec![STX].iter().cloned());
        // Append crc and ETX
        serialized.push(crc);
        serialized.push(ETX);
        serialized
    }

    pub fn deserialize<'a, T>(&mut self, data: &'a Vec<u8>) -> T
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
        let mut pg = PacketGenerator::default();
        let content = pg.get_info();

        assert_eq!(
            pg.serialize(content),
            vec![STX, 0x01, 0x01, 0x49, 0xB5, ETX]
        );
    }

    #[test]
    fn get_version() {
        let mut pg = PacketGenerator::default();
        let content = pg.get_version();

        assert_eq!(
            pg.serialize(content),
            vec![STX, 0x01, 0x01, 0x56, 0x69, ETX]
        );
    }

    #[test]
    fn get_version_response() {
        let mut pg = PacketGenerator::default();

        assert_eq!(
            pg.serialize(Content {
                uc: 1,
                id: 1,
                command: COMMAND_GET_VERSION,
                p: GetVersionResponse {
                    dv3: 2,
                    dv2: 3,
                    dv01: 1113,
                    av3: 2,
                    av2: 3,
                    av01: 1113,
                    name: "Test".to_string(),
                    sn: vec![1, 2, 3, 4],
                },
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
        let mut pg = PacketGenerator::default();
        let p = pg.write_register(10, 0xF0, vec![1, 2, 3, 4]);

        assert_eq!(
            pg.serialize(p),
            vec![STX, 0x01, 0x01, 0x57, 10, 0, 0, 0, 0xF0, 4, 1, 2, 3, 4, 0xB6, ETX]
        );
    }

    #[test]
    fn get_info_serialized() {
        let mut pg = PacketGenerator::default();

        let p = pg.get_info();
        let serialized = pg.serialize(&p);
        let deserialize: Packet<Content<Generic>> = pg.deserialize(&serialized);

        assert_eq!(p, deserialize.content);
    }

    #[test]
    fn write_register_serialized() {
        let mut pg = PacketGenerator::default();
        let p = pg.write_register(10, 0xF0, vec![1, 2, 3, 4]);
        let serialized = pg.serialize(&p);
        let deserialize: Packet<Content<WriteRegister>> = pg.deserialize(&serialized);

        assert_eq!(p, deserialize.content);
    }
}
