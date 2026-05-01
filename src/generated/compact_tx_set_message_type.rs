#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// CompactTxSetMessageType is an XDR Enum defined as:
///
/// ```text
/// enum CompactTxSetMessageType
/// {
///     COMPACT_TX_SET = 0,
///     COMPACT_TX_SET_GET = 1,
///     COMPACT_TX_SET_GET_TXS = 2,
///     COMPACT_TX_SET_TXS = 3
/// };
/// ```
///
// enum
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[repr(i32)]
pub enum CompactTxSetMessageType {
    #[cfg_attr(feature = "alloc", default)]
    Set = 0,
    SetGet = 1,
    SetGetTxs = 2,
    SetTxs = 3,
}

impl CompactTxSetMessageType {
    const _VARIANTS: &[CompactTxSetMessageType] = &[
        CompactTxSetMessageType::Set,
        CompactTxSetMessageType::SetGet,
        CompactTxSetMessageType::SetGetTxs,
        CompactTxSetMessageType::SetTxs,
    ];
    pub const VARIANTS: [CompactTxSetMessageType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Set", "SetGet", "SetGetTxs", "SetTxs"];
    pub const VARIANTS_STR: [&'static str; Self::_VARIANTS_STR.len()] = {
        let mut arr = [Self::_VARIANTS_STR[0]; Self::_VARIANTS_STR.len()];
        let mut i = 1;
        while i < Self::_VARIANTS_STR.len() {
            arr[i] = Self::_VARIANTS_STR[i];
            i += 1;
        }
        arr
    };

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Set => "Set",
            Self::SetGet => "SetGet",
            Self::SetGetTxs => "SetGetTxs",
            Self::SetTxs => "SetTxs",
        }
    }

    #[must_use]
    pub const fn variants() -> [CompactTxSetMessageType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for CompactTxSetMessageType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<CompactTxSetMessageType> for CompactTxSetMessageType {
    fn variants() -> slice::Iter<'static, CompactTxSetMessageType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for CompactTxSetMessageType {}

impl fmt::Display for CompactTxSetMessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for CompactTxSetMessageType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => CompactTxSetMessageType::Set,
            1 => CompactTxSetMessageType::SetGet,
            2 => CompactTxSetMessageType::SetGetTxs,
            3 => CompactTxSetMessageType::SetTxs,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<CompactTxSetMessageType> for i32 {
    #[must_use]
    fn from(e: CompactTxSetMessageType) -> Self {
        e as Self
    }
}

impl ReadXdr for CompactTxSetMessageType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for CompactTxSetMessageType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
