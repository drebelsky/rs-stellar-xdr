#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// CompactTxSetGetTxs is an XDR Struct defined as:
///
/// ```text
/// struct CompactTxSetGetTxs
/// {
///     Hash txSetHash;
///     // differentially encoded indices of transactions requested
///     opaque indices<>;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct CompactTxSetGetTxs {
    pub tx_set_hash: Hash,
    pub indices: BytesM,
}

impl ReadXdr for CompactTxSetGetTxs {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_set_hash: Hash::read_xdr(r)?,
                indices: BytesM::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for CompactTxSetGetTxs {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_set_hash.write_xdr(w)?;
            self.indices.write_xdr(w)?;
            Ok(())
        })
    }
}
