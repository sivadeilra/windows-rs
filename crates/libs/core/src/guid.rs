#![allow(clippy::many_single_char_names)]

mod uuid_integration;

use super::*;
use core::str::FromStr;

/// A globally unique identifier ([GUID](https://docs.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid))
/// used to identify COM and WinRT interfaces.
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy_derive::AsBytes,
        zerocopy_derive::FromZeroes,
        zerocopy_derive::FromBytes
    )
)]
pub struct GUID {
    /// Specifies the first 8 hexadecimal digits.
    pub data1: u32,

    /// Specifies the first group of 4 hexadecimal digits.
    pub data2: u16,

    /// Specifies the second group of 4 hexadecimal digits.
    pub data3: u16,

    /// The first 2 bytes contain the third group of 4 hexadecimal digits. The remaining 6 bytes contain the final 12 hexadecimal digits.
    pub data4: [u8; 8],
}

impl GUID {
    /// Creates a unique `GUID` value.
    pub fn new() -> Result<Self> {
        unsafe { imp::CoCreateGuid() }
    }

    /// Creates a `GUID` represented by the all-zero byte-pattern.
    pub const fn zeroed() -> Self {
        Self {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    /// Creates a `GUID` with the given constant values.
    pub const fn from_values(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }

    /// Creates a `GUID` from a `u128` value.
    pub const fn from_u128(uuid: u128) -> Self {
        Self {
            data1: (uuid >> 96) as u32,
            data2: (uuid >> 80 & 0xffff) as u16,
            data3: (uuid >> 64 & 0xffff) as u16,
            data4: (uuid as u64).to_be_bytes(),
        }
    }

    /// Converts a `GUID` to a `u128` value.
    pub const fn to_u128(&self) -> u128 {
        ((self.data1 as u128) << 96)
            + ((self.data2 as u128) << 80)
            + ((self.data3 as u128) << 64)
            + u64::from_be_bytes(self.data4) as u128
    }

    /// Creates a `GUID` for a "generic" WinRT type.
    pub const fn from_signature(signature: imp::ConstBuffer) -> Self {
        let data = imp::ConstBuffer::from_slice(&[
            0x11, 0xf4, 0x7a, 0xd5, 0x7b, 0x73, 0x42, 0xc0, 0xab, 0xae, 0x87, 0x8b, 0x1e, 0x16,
            0xad, 0xee,
        ]);

        let data = data.push_other(signature);

        let bytes = imp::sha1(&data).bytes();
        let first = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

        let second = u16::from_be_bytes([bytes[4], bytes[5]]);
        let mut third = u16::from_be_bytes([bytes[6], bytes[7]]);
        third = (third & 0x0fff) | (5 << 12);
        let fourth = (bytes[8] & 0x3f) | 0x80;

        Self::from_values(
            first,
            second,
            third,
            [
                fourth, bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
            ],
        )
    }

    /// Creates a `GUID` from a serialized byte array. The fields are stored in little-endian
    /// byte order.
    pub const fn from_bytes_le(bytes: [u8; 16]) -> Self {
        Self {
            data1: u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            data2: u16::from_le_bytes([bytes[4], bytes[5]]),
            data3: u16::from_le_bytes([bytes[6], bytes[7]]),
            data4: [
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                bytes[15],
            ],
        }
    }
}

#[test]
fn test_from_bytes() {
    assert_eq!(
        GUID::from_u128(0x12345678_abcd_ee33_f0f1_f2f3f4f5f6f7),
        GUID::from_bytes_le([
            0x78, 0x56, 0x34, 0x12, 0xcd, 0xab, 0x33, 0xee, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5,
            0xf6, 0xf7
        ])
    );
}

impl RuntimeType for GUID {
    const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice(b"g16");
}

impl TypeKind for GUID {
    type TypeKind = CopyType;
}

impl core::fmt::Debug for GUID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}

impl core::fmt::Display for GUID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:08X?}-{:04X?}-{:04X?}-{:02X?}{:02X?}-{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7]
        )
    }
}

/// This error type is used for parsing GUIDs from strings.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct InvalidGuidError;

impl FromStr for GUID {
    type Err = InvalidGuidError;

    fn from_str(mut value: &str) -> core::result::Result<Self, Self::Err> {
        if value.len() == 38 {
            if let Some(s) = value.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
                value = s;
            }
        }

        if value.len() != 36 {
            return Err(InvalidGuidError);
        }

        let mut bytes = value.bytes();

        let a = u32::from_be_bytes([
            bytes.next_u8()?,
            bytes.next_u8()?,
            bytes.next_u8()?,
            bytes.next_u8()?,
        ]);

        if bytes.next() != Some(b'-') {
            return Err(InvalidGuidError);
        }

        let b = u16::from_be_bytes([bytes.next_u8()?, bytes.next_u8()?]);

        if bytes.next() != Some(b'-') {
            return Err(InvalidGuidError);
        }

        let c = u16::from_be_bytes([bytes.next_u8()?, bytes.next_u8()?]);

        if bytes.next() != Some(b'-') {
            return Err(InvalidGuidError);
        }

        let d = bytes.next_u8()?;
        let e = bytes.next_u8()?;
        if bytes.next() != Some(b'-') {
            return Err(InvalidGuidError);
        }

        let f = bytes.next_u8()?;
        let g = bytes.next_u8()?;
        let h = bytes.next_u8()?;
        let i = bytes.next_u8()?;
        let j = bytes.next_u8()?;
        let k = bytes.next_u8()?;

        Ok(Self::from_values(a, b, c, [d, e, f, g, h, i, j, k]))
    }
}

impl From<&str> for GUID {
    fn from(value: &str) -> Self {
        GUID::from_str(value).expect("Invalid GUID string")
    }
}

#[test]
fn guid_from_str() {
    assert!(GUID::from_str("").is_err());

    assert_eq!(
        GUID::from_str("a1a2a3a4-b1b2-c1c2-d1d2-e1e2e3e4e5e6").unwrap(),
        GUID::from_u128(0xa1a2a3a4_b1b2_c1c2_d1d2_e1e2e3e4e5e6)
    );
    assert_eq!(
        GUID::from_str("{a1a2a3a4-b1b2-c1c2-d1d2-e1e2e3e4e5e6}").unwrap(),
        GUID::from_u128(0xa1a2a3a4_b1b2_c1c2_d1d2_e1e2e3e4e5e6)
    );

    // has { but no matching }
    assert!(GUID::from_str("{a1a2a3a4-b1b2-c1c2-d1d2-e1e2e3e4e5e6").is_err());

    // has wrong separators
    assert!(GUID::from_str("gggggggg_b1b2_c1c2_d1d2_e1e2e3e4e5e6").is_err());

    // safely fail on non-ASCII input
    let non_ascii = "0000\u{1f643}0000000000000000000000000000";
    assert_eq!(non_ascii.len(), 36); // so that it passes the length check
    assert!(GUID::from_str(non_ascii).is_err());
}

impl From<u128> for GUID {
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}

impl From<GUID> for u128 {
    fn from(value: GUID) -> Self {
        value.to_u128()
    }
}

trait HexReader {
    fn next_u4(&mut self) -> core::result::Result<u8, InvalidGuidError>;
    fn next_u8(&mut self) -> core::result::Result<u8, InvalidGuidError> {
        let hi = self.next_u4()?;
        let lo = self.next_u4()?;
        Ok((hi << 4) | lo)
    }
}

impl HexReader for core::str::Bytes<'_> {
    fn next_u4(&mut self) -> core::result::Result<u8, InvalidGuidError> {
        let value = self.next().ok_or(InvalidGuidError)?;
        match value {
            b'0'..=b'9' => Ok(value - b'0'),
            b'A'..=b'F' => Ok(10 + value - b'A'),
            b'a'..=b'f' => Ok(10 + value - b'a'),
            _ => Err(InvalidGuidError),
        }
    }
}
