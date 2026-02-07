use bytes::{Bytes, BytesMut};

use crate::TypePath;

impl TypePath for Bytes {
    fn type_path() -> &'static str {
        "bytes::Bytes"
    }
}

impl TypePath for BytesMut {
    fn type_path() -> &'static str {
        "bytes::BytesMut"
    }
}
