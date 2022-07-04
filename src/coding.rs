use std::io::{Error, ErrorKind};

pub fn hex_encode<T: AsRef<[u8]>>(src: T) -> String {
    faster_hex::hex_string(src.as_ref())
}

pub fn hex_decode(src: &str) -> Result<Vec<u8>, Error> {
    if src.is_empty() {
        return Ok(Vec::new());
    }

    let src = src.as_bytes();
    let mut ret = vec![0u8; src.len() / 2];
    faster_hex::hex_decode(src, &mut ret)
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    Ok(ret)
}
