#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// CompactTxSet is an XDR Struct defined as:
///
/// ```text
/// struct CompactTxSet
/// {
///     Hash txSetHash; // hash of the full tx set
///     Hash previousLedgerHash;
///     int64* baseFee;
///     // 6 byte siphashes
///     opaque txs<>;
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
pub struct CompactTxSet {
    pub tx_set_hash: Hash,
    pub previous_ledger_hash: Hash,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "Option<NumberOrString>")
    )]
    pub base_fee: Option<i64>,
    pub txs: BytesM,
}

impl ReadXdr for CompactTxSet {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_set_hash: Hash::read_xdr(r)?,
                previous_ledger_hash: Hash::read_xdr(r)?,
                base_fee: Option::<i64>::read_xdr(r)?,
                txs: BytesM::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for CompactTxSet {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_set_hash.write_xdr(w)?;
            self.previous_ledger_hash.write_xdr(w)?;
            self.base_fee.write_xdr(w)?;
            self.txs.write_xdr(w)?;
            Ok(())
        })
    }
}
