//! This module provides conversions between the Windows [`GUID`] type and the `uuid::Uuid` type.
//!
//! These conversions are enabled by the optional dependency on the `uuid` crate, which is
//! activated in the same way that a crate feature is activated.
//!
//! `GUID` and `Uuid` have different in-memory byte representations; they cannot be the same type
//! and they cannot be bitcast (transmuted).
//!
//! `GUID` contains fields that are larger than a single byte and these fields use host-endian form,
//! which in practice means little-endian.  `Uuid` stores its data in a single `[u8; 16]` byte
//! array, which is in big-endian form.  This also means that the types have different alignment
//! requirements (`GUID` is 4-byte aligned; `Uuid` is 1-byte aligned).

#![cfg(feature = "uuid")]

use super::GUID;
use uuid::Uuid;

impl From<Uuid> for GUID {
    fn from(uuid: Uuid) -> Self {
        let (a, b, c, d) = uuid.as_fields();
        Self {
            data1: a,
            data2: b,
            data3: c,
            data4: *d,
        }
    }
}

#[test]
fn guid_from_uuid() {
    let u = Uuid::from_fields(
        0x12345678,
        0xabcd,
        0xeeee,
        &[0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7],
    );
    let g = GUID::from(u);

    assert_eq!(
        format!("{}", u.hyphenated()).to_ascii_uppercase(),
        format!("{g}").to_ascii_uppercase()
    );
}

impl From<GUID> for Uuid {
    fn from(guid: GUID) -> Self {
        Uuid::from_fields(guid.data1, guid.data2, guid.data3, &guid.data4)
    }
}

#[test]
fn uuid_from_guid() {
    let g = GUID::from_u128(0x12345678_abcd_eeee_f0f1_f2f3f4f5f6f7);
    let u = Uuid::from(g);

    assert_eq!(
        format!("{}", u.hyphenated()).to_ascii_uppercase(),
        format!("{g}").to_ascii_uppercase()
    );
}

impl PartialEq<Uuid> for GUID {
    fn eq(&self, other: &Uuid) -> bool {
        *self == GUID::from(*other)
    }
}

impl PartialEq<GUID> for Uuid {
    fn eq(&self, other: &GUID) -> bool {
        *self == Uuid::from(*other)
    }
}
