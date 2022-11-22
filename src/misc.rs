use systemstat::ByteSize;

pub(crate) struct Misc;

impl Misc {
    pub(crate) fn pct_from_byte_sizes(used: ByteSize, total: ByteSize) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let pct: f64 = (used.as_u64() as f64 / total.as_u64() as f64) * 100.0;

        pct
    }
}
