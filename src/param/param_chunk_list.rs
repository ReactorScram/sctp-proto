use super::{param_header::*, param_type::*, *};
use crate::chunk::chunk_type::*;

use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ParamChunkList {
    pub(crate) chunk_types: Vec<ChunkType>,
}

impl fmt::Display for ParamChunkList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.header(),
            self.chunk_types
                .iter()
                .map(|ct| ct.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Param for ParamChunkList {
    fn header(&self) -> ParamHeader {
        ParamHeader {
            typ: ParamType::ChunkList,
            value_length: self.value_length() as u16,
        }
    }

    fn unmarshal(raw: &Bytes) -> Result<Self, Error> {
        let header = ParamHeader::unmarshal(raw)?;

        if header.typ != ParamType::ChunkList {
            return Err(Error::ErrParamTypeUnexpected);
        }

        let reader = &mut raw.slice(PARAM_HEADER_LENGTH..);

        let mut chunk_types = vec![];
        while reader.has_remaining() {
            chunk_types.push(reader.get_u8().into())
        }

        Ok(ParamChunkList { chunk_types })
    }

    fn marshal_to(&self, buf: &mut BytesMut) -> Result<usize, Error> {
        self.header().marshal_to(buf)?;
        for ct in &self.chunk_types {
            buf.put_u8(*ct as u8);
        }
        Ok(buf.len())
    }

    fn value_length(&self) -> usize {
        self.chunk_types.len()
    }
}