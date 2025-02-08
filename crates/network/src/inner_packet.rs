#[derive(Clone)]
pub struct InnerPacket(Vec<u8>);

impl InnerPacket {
    const HEAD_MAGIC: u32 = 0x12348765;
    const TAIL_MAGIC: u32 = 0x56784321;

    pub fn build(conv: u32, token: u32, uid: u32, cmd_id: u16, id: u16, data: Vec<u8>) -> Self {
        use byteorder::{ByteOrder, BE};

        let size = data.len() + 26;
        let mut buf = vec![0; size];

        BE::write_u32(&mut buf[0..4], Self::HEAD_MAGIC);
        BE::write_u32(&mut buf[4..8], conv);
        BE::write_u32(&mut buf[8..12], token);
        BE::write_u32(&mut buf[12..16], uid);
        BE::write_u16(&mut buf[16..18], cmd_id);
        BE::write_u16(&mut buf[18..20], id);
        // sequence_id: buf[20] = 0u8, sequence_num: buf[21] = 0u8
        buf[20..22].fill(0);
        buf[22..(22 + data.len())].copy_from_slice(&data);
        BE::write_u32(&mut buf[(22 + data.len())..], Self::TAIL_MAGIC);

        Self(buf)
    }

    pub fn set_sequence(&mut self, seq_id: u8, seq_num: u8) {
        self.0[20] = seq_id;
        self.0[21] = seq_num;
    }

    pub fn get_sequence(&self) -> (u8, u8) {
        (self.0[20], self.0[21])
    }

    pub fn get_conv(&self) -> u32 {
        u32::from_be_bytes(self.0[4..8].try_into().unwrap())
    }

    pub fn get_token(&self) -> u32 {
        u32::from_be_bytes(self.0[8..12].try_into().unwrap())
    }

    pub fn get_uid(&self) -> u32 {
        u32::from_be_bytes(self.0[12..16].try_into().unwrap())
    }

    pub fn get_cmd_id(&self) -> u16 {
        u16::from_be_bytes(self.0[16..18].try_into().unwrap())
    }
    
    pub fn get_id(&self) -> u16 {
        u16::from_be_bytes(self.0[18..20].try_into().unwrap())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.0[22..self.0.len() - 4]
    }

    pub fn is_valid(&self) -> bool {
        self.0.len() >= 16
            && self.0[0..4] == Self::HEAD_MAGIC.to_be_bytes()
            && self.0[self.0.len() - 4..] == Self::TAIL_MAGIC.to_be_bytes()
    }

    pub fn from_bytes(buf: Vec<u8>) -> Option<Self> {
        let r= Self(buf);

        if r.is_valid() {
            Some(r)
        } else {
            None
        }
    }

    pub fn to_raw(&self) -> Box<[u8]> {
        Box::from(self.0.clone())
    }
}
