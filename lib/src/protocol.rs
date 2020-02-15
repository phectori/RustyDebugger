use crate::packet::*;

pub struct Protocol {
    data: Vec<u8>,
    response: Vec<u8>,
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol {
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
            let p: Packet<Content<GetVersionResponse>> = PacketGenerator::deserialize(&data);
            println!("Received {:?}", p);
        }
        else if command == COMMAND_WRITE_REGISTER {
            let p: Packet<Content<WriteRegisterResponse>> = PacketGenerator::deserialize(&data);
            println!("Received {:?}", p);
        }
    }

    /// Processing incomming messages on the host side (application/microcontroller)
    pub fn process_packet_host(&mut self, data: Vec<u8>) {
        let command = data[3];

        if command == COMMAND_GET_VERSION {
            let p: Packet<Content<Generic>> = PacketGenerator::deserialize(&data);
            println!("Received {:?}", p);

            let r = GetVersionResponse {
                command: COMMAND_GET_VERSION,
                dv3: 2,
                dv2: 3,
                dv01: 1113,
                av3: 2,
                av2: 3,
                av01: 1113,
                name: "Test".to_string(),
                sn: vec![1, 2, 3, 4],
            };
            println!("Responded with {:?}", r);

            let mut response = PacketGenerator::serialize(r);
            
            self.response.append(&mut response);
        }
        else if command == COMMAND_WRITE_REGISTER {
            let p: Packet<Content<WriteRegister>> = PacketGenerator::deserialize(&data);
            println!("Received {:?}", p);
        }
    }

    pub fn take_response(&mut self) -> Vec<u8> {
        let r = self.response.to_vec();
        self.response.clear();
        r
    }
}
