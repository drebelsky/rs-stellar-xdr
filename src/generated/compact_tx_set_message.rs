#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// CompactTxSetMessage is an XDR Union defined as:
///
/// ```text
/// union CompactTxSetMessage switch (CompactTxSetMessageType type)
/// {
/// case COMPACT_TX_SET:
///     CompactTxSet compactTxSet;
/// case COMPACT_TX_SET_GET:
///     CompactTxSetGet compactTxSetGet;
/// case COMPACT_TX_SET_GET_TXS:
///     CompactTxSetGetTxs compactTxSetGetTxs;
/// case COMPACT_TX_SET_TXS:
///     CompactTxSetTxs compactTxSetTxs;
/// };
/// ```
///
// union with discriminant CompactTxSetMessageType
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[allow(clippy::large_enum_variant)]
pub enum CompactTxSetMessage {
    Set(CompactTxSet),
    SetGet(CompactTxSetGet),
    SetGetTxs(CompactTxSetGetTxs),
    SetTxs(CompactTxSetTxs),
}

#[cfg(feature = "alloc")]
impl Default for CompactTxSetMessage {
    fn default() -> Self {
        Self::Set(CompactTxSet::default())
    }
}

impl CompactTxSetMessage {
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
            Self::Set(_) => "Set",
            Self::SetGet(_) => "SetGet",
            Self::SetGetTxs(_) => "SetGetTxs",
            Self::SetTxs(_) => "SetTxs",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> CompactTxSetMessageType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Set(_) => CompactTxSetMessageType::Set,
            Self::SetGet(_) => CompactTxSetMessageType::SetGet,
            Self::SetGetTxs(_) => CompactTxSetMessageType::SetGetTxs,
            Self::SetTxs(_) => CompactTxSetMessageType::SetTxs,
        }
    }

    #[must_use]
    pub const fn variants() -> [CompactTxSetMessageType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for CompactTxSetMessage {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<CompactTxSetMessageType> for CompactTxSetMessage {
    #[must_use]
    fn discriminant(&self) -> CompactTxSetMessageType {
        Self::discriminant(self)
    }
}

impl Variants<CompactTxSetMessageType> for CompactTxSetMessage {
    fn variants() -> slice::Iter<'static, CompactTxSetMessageType> {
        Self::VARIANTS.iter()
    }
}

impl Union<CompactTxSetMessageType> for CompactTxSetMessage {}

impl ReadXdr for CompactTxSetMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: CompactTxSetMessageType = <CompactTxSetMessageType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                CompactTxSetMessageType::Set => Self::Set(CompactTxSet::read_xdr(r)?),
                CompactTxSetMessageType::SetGet => Self::SetGet(CompactTxSetGet::read_xdr(r)?),
                CompactTxSetMessageType::SetGetTxs => {
                    Self::SetGetTxs(CompactTxSetGetTxs::read_xdr(r)?)
                }
                CompactTxSetMessageType::SetTxs => Self::SetTxs(CompactTxSetTxs::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for CompactTxSetMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Set(v) => v.write_xdr(w)?,
                Self::SetGet(v) => v.write_xdr(w)?,
                Self::SetGetTxs(v) => v.write_xdr(w)?,
                Self::SetTxs(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
