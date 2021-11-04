// These functions are necessarily inefficient, because they must revert
// encoding conversions performed by the standard library. However, there is
// currently no better alternative.

use std::borrow::Cow;
use std::error::Error;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;
use std::result;
use std::str;

if_raw! {
    pub(super) mod raw;
}

mod wtf8;
use wtf8::DecodeWide;
use wtf8::EncodeWide;

#[derive(Debug, Eq, PartialEq)]
pub(super) enum EncodingError {
    Byte(u8),
    CodePoint(u32),
    End(),
}

impl EncodingError {
    fn position(&self) -> Cow<'_, str> {
        // Variants are not recognized on type aliases in older versions:
        // https://github.com/rust-lang/rust/pull/61682
        match self {
            EncodingError::Byte(byte) => {
                Cow::Owned(format!("byte b'\\x{:02X}'", byte))
            }
            EncodingError::CodePoint(code_point) => {
                Cow::Owned(format!("code point U+{:04X}", code_point))
            }
            EncodingError::End() => Cow::Borrowed("end of string"),
        }
    }
}

impl Display for EncodingError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "os_str_bytes: byte sequence is not representable in the platform \
            encoding; error at {}",
            self.position(),
        )
    }
}

impl Error for EncodingError {}

type Result<T> = result::Result<T, EncodingError>;

pub(crate) fn os_str_from_bytes(string: &[u8]) -> Result<Cow<'_, OsStr>> {
    Ok(Cow::Owned(os_string_from_bytes(string)?))
}

pub(crate) fn os_str_to_bytes(os_string: &OsStr) -> Cow<'_, [u8]> {
    Cow::Owned(DecodeWide::new(OsStrExt::encode_wide(os_string)).collect())
}

pub(crate) fn os_string_from_bytes(string: &[u8]) -> Result<OsString> {
    let encoder = EncodeWide::new(string.iter().map(|&x| x));

    // Collecting an iterator into a result ignores the size hint:
    // https://github.com/rust-lang/rust/issues/48994
    let mut encoded_string = Vec::with_capacity(encoder.size_hint().0);
    for wchar in encoder {
        encoded_string.push(wchar?);
    }
    Ok(OsStringExt::from_wide(&encoded_string))
}

pub(crate) fn os_string_from_vec(string: Vec<u8>) -> Result<OsString> {
    os_string_from_bytes(&string)
}

pub(crate) fn os_string_into_vec(os_string: OsString) -> Vec<u8> {
    os_str_to_bytes(&os_string).into_owned()
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use crate::OsStrBytes;

    use super::EncodingError;

    #[test]
    fn test_invalid() {
        use EncodingError::Byte;
        use EncodingError::CodePoint;
        use EncodingError::End;

        test_error(Byte(b'\x83'), b"\x0C\x83\xD7\x3E");
        test_error(Byte(b'\x52'), b"\x19\xF7\x52\x84");
        test_error(Byte(b'\xB8'), b"\x70\xB8\x1F\x66");
        test_error(CodePoint(0x34_0388), b"\x70\xFD\x80\x8E\x88");
        test_error(Byte(b'\x80'), b"\x80");
        test_error(Byte(b'\x80'), b"\x80\x80");
        test_error(Byte(b'\x80'), b"\x80\x80\x80");
        test_error(Byte(b'\x81'), b"\x81");
        test_error(Byte(b'\x88'), b"\x88\xB4\xC7\x46");
        test_error(Byte(b'\x97'), b"\x97\xCE\x06");
        test_error(Byte(b'\x00'), b"\xC2\x00");
        test_error(Byte(b'\x7F'), b"\xC2\x7F");
        test_error(Byte(b'\x09'), b"\xCD\x09\x95");
        test_error(Byte(b'\x43'), b"\xCD\x43\x5F\xA0");
        test_error(Byte(b'\x69'), b"\xD7\x69\xB2");
        test_error(CodePoint(0x528), b"\xE0\x94\xA8");
        test_error(CodePoint(0x766), b"\xE0\x9D\xA6\x12\xAE");
        test_error(Byte(b'\xFD'), b"\xE2\xAB\xFD\x51");
        test_error(Byte(b'\xC4'), b"\xE3\xC4");
        test_error(CodePoint(0xDC00), b"\xED\xA0\x80\xED\xB0\x80");
        test_error(End(), b"\xF1");
        test_error(End(), b"\xF1\x80");
        test_error(End(), b"\xF1\x80\x80");
        test_error(Byte(b'\xF1'), b"\xF1\x80\x80\xF1");
        test_error(CodePoint(0x15_EC46), b"\xF5\x9E\xB1\x86");
        test_error(End(), b"\xFB");
        test_error(End(), b"\xFB\x80");
        test_error(End(), b"\xFB\x80\x80");
        test_error(CodePoint(0x2C_0000), b"\xFB\x80\x80\x80");
        test_error(End(), b"\xFF");
        test_error(End(), b"\xFF\x80");
        test_error(End(), b"\xFF\x80\x80");
        test_error(CodePoint(0x3C_0000), b"\xFF\x80\x80\x80");
        test_error(CodePoint(0x3C_6143), b"\xFF\x86\x85\x83");

        fn test_error(error: EncodingError, string: &[u8]) {
            use crate::EncodingError;

            assert_eq!(
                Err(error),
                OsStr::from_bytes(string).map_err(|EncodingError(x)| x),
            );
        }
    }
}
