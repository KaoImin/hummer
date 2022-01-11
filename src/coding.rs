use hex_simd::{decode_to_boxed_bytes, encode_to_boxed_str, AsciiCase, Error};

pub fn hex_encode<T: AsRef<[u8]>>(src: T) -> String {
    encode_to_boxed_str(src.as_ref(), AsciiCase::Lower).into_string()
}

pub fn hex_decode(src: &str) -> Result<Vec<u8>, Error> {
    decode_to_boxed_bytes(src.as_bytes()).map(Into::into)
}

pub fn hex_encode_upper<T: AsRef<[u8]>>(src: T) -> String {
    encode_to_boxed_str(src.as_ref(), AsciiCase::Upper).into_string()
}
