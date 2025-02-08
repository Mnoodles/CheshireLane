use anyhow::Result;
use byteorder::{BigEndian, ByteOrder};
use proto::{ProtoMsg, CheshireMessage};

#[derive(thiserror::Error, Debug)]
enum PacketError {
    #[error("Packet length is too short. Minimum required {0} bytes, but received {1} bytes.")]
    PacketSizeTooShort(usize, usize),
    #[error("Packet length ({0}) does not match the actual data length ({1}).")]
    PacketLenMismatch(usize, usize),
    #[error("Error in parsing packet data.")]
    ParsingPacketDataFailure,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Packet {
    pub length: u16,
    pub flag: u8,
    pub cmd_id: u16,
    pub id: u16,
    #[serde(skip_serializing)]
    pub data: Vec<u8>,
}

impl Packet {
    const LENGTH_SIZE: usize = 2;
    const HEADER_SIZE: usize = 5;

    pub fn new(data: Vec<u8>) -> Result<Packet> {
        // let length = u16::from_be_bytes([data[0], data[1]]);
        // let flag = data[Self::LENGTH_SIZE];
        // let cmd_id = u16::from_be_bytes([
        //     data[Self::LENGTH_SIZE + 1],
        //     data[Self::LENGTH_SIZE + 2],
        // ]);
        // let id = u16::from_be_bytes([
        //     data[Self::HEADER_SIZE],
        //     data[Self::HEADER_SIZE + 1],
        // ]);
        // let data = data[Self::HEADER_SIZE + Self::LENGTH_SIZE..length as usize]
        //     .to_vec();

        if data.len() < Self::HEADER_SIZE {
            return Err(PacketError::PacketSizeTooShort(
                data.len(), Self::HEADER_SIZE).into());
        }

        let length = BigEndian::read_u16(&data[0..Self::LENGTH_SIZE]);
        if data.len() != (length as usize + Self::LENGTH_SIZE) {
            return Err(PacketError::PacketLenMismatch(
                length as usize, data.len()).into());
        }

        let flag = data[Self::LENGTH_SIZE];
        let cmd_id = BigEndian::read_u16(
            &data[Self::LENGTH_SIZE + 1..Self::HEADER_SIZE]);
        let id = BigEndian::read_u16(
            &data[Self::HEADER_SIZE..Self::HEADER_SIZE + 2]);

        let start = Self::HEADER_SIZE + Self::LENGTH_SIZE;
        let data_len = (length as usize).saturating_sub(Self::HEADER_SIZE);
        let data = data
            .get(start..start + data_len)
            .ok_or(PacketError::ParsingPacketDataFailure)?
            .to_vec();

        Ok(Packet {
            length,
            flag,
            cmd_id,
            id,
            data,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::with_capacity(
            self.length as usize + Self::LENGTH_SIZE);

        let mut length_bytes = [0u8; 2];
        BigEndian::write_u16(&mut length_bytes, self.length);
        bytes.extend_from_slice(&length_bytes);

        bytes.push(self.flag);

        let mut cmd_id_bytes = [0u8; 2];
        BigEndian::write_u16(&mut cmd_id_bytes, self.cmd_id);
        bytes.extend_from_slice(&cmd_id_bytes);

        let mut id_bytes = [0u8; 2];
        BigEndian::write_u16(&mut id_bytes, self.id);
        bytes.extend_from_slice(&id_bytes);

        bytes.extend_from_slice(&self.data);

        Ok(bytes)
    }

    pub fn encode<T: CheshireMessage>(proto: &T, id: u16) -> Self {
        let data = proto.encode_to_vec();
        Self {
            length: (data.len() + Self::HEADER_SIZE) as u16,
            flag: 0,
            cmd_id: proto.get_cmd_id(),
            id,
            data,
        }
    }

    pub fn encode_raw(cmd_id: u16, id: u16, data: Vec<u8>) -> Self {
        Self {
            length: (data.len() + Self::HEADER_SIZE) as u16,
            flag: 0,
            cmd_id,
            id,
            data,
        }
    }

    pub fn decode<T: ProtoMsg + Default>(&self) -> Option<T> {
        T::decode(self.data.as_slice()).ok()
    }
}
