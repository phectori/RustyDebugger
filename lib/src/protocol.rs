use crate::packet::*;

pub struct Protocol {
    pg: PacketGenerator,
    data: Vec<u8>,
    response: Vec<u8>,
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol {
            pg: PacketGenerator::default(),
            data: Vec::new(),
            response: Vec::new(),
        }
    }
}

impl Protocol {
    pub fn process_data(&mut self, mut data: Vec<u8>) -> Option<Vec<u8>> {
        // Todo: Rewrite this code.
        // It not safe.

        // Append the data
        self.data.append(&mut data);

        // Search for the STX byte
        if let Some(stx_index) = self.data.iter().position(|&x| x == STX) {
            // Now look for the ETX byte
            if let Some(etx_index) = self.data.iter().position(|&x| x == ETX) {
                let packet_data = self.data[stx_index..etx_index + 1].to_vec();

                // Remove processed data
                for _x in 0..etx_index + 1 {
                    self.data.remove(0);
                }

                return Some(packet_data);
            }
            return None;
        } else {
            return None;
        }
    }

    // Processing incomming messages on the client side (Debugger side)
    pub fn process_packet(&mut self, data: Vec<u8>) {
        let command = data[3];

        if command == COMMAND_GET_VERSION {
            let p: Packet<Content<GetVersionHost>> = self.pg.deserialize(&data);
            println!("Received {:?}", p.content);
        } else if command == COMMAND_WRITE_REGISTER {
            let p: Packet<Content<WriteRegisterHost>> = self.pg.deserialize(&data);
            println!("Received {:?}", p.content);
        } else if command == COMMAND_READ_CHANNEL_DATA {
            // let p: Packet<Content<WriteRegisterHost>> = self.pg.deserialize(&data);
            // println!("Received {:?}", p.content);
        }
    }

    /// Processing incomming messages on the host side (application/microcontroller)
    pub fn process_packet_host(&mut self, data: Vec<u8>) {
        let command = data[3];

        if command == COMMAND_GET_VERSION {
            let p: Packet<Content<Generic>> = self.pg.deserialize(&data);
            println!("Received {:?}", p.content);

            let mut r = self.pg.get_version_response();
            self.response.append(&mut r);
        } else if command == COMMAND_WRITE_REGISTER {
            let p: Packet<Content<WriteRegister>> = self.pg.deserialize(&data);
            println!("Received {:?}", p.content);
        }
    }

    pub fn take_response(&mut self) -> Vec<u8> {
        let r = self.response.to_vec();
        self.response.clear();
        r
    }
}
