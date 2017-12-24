// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
#[allow(unused_imports)]
use string::*;
#[allow(unused_imports)]
use data_types::*;
#[allow(unused_imports)]
use data_value::*;
#[allow(unused_imports)]
use date_time::*;
#[allow(unused_imports)]
use node_id::*;
#[allow(unused_imports)]
use service_types::enums::*;
#[allow(unused_imports)]
use variant::*;
#[allow(unused_imports)]
use service_types::impls::*;
#[allow(unused_imports)]
use node_ids::ObjectId;
#[allow(unused_imports)]
use status_codes::StatusCode;
use service_types::ReferenceDescription;

/// The result of a browse operation.
#[derive(Debug, Clone, PartialEq)]
pub struct BrowseResult {
    pub status_code: StatusCode,
    pub continuation_point: ByteString,
    pub references: Option<Vec<ReferenceDescription>>,
}

impl MessageInfo for BrowseResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::BrowseResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<BrowseResult> for BrowseResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.status_code.byte_len();
        size += self.continuation_point.byte_len();
        size += byte_len_array(&self.references);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.status_code.encode(stream)?;
        size += self.continuation_point.encode(stream)?;
        size += write_array(stream, &self.references)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let status_code = StatusCode::decode(stream)?;
        let continuation_point = ByteString::decode(stream)?;
        let references: Option<Vec<ReferenceDescription>> = read_array(stream)?;
        Ok(BrowseResult {
            status_code,
            continuation_point,
            references,
        })
    }
}