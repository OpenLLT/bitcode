use crate::convert::{impl_convert, ConvertFrom};
use jiff::Timestamp;

type TimestampConversion = i128;

impl ConvertFrom<&Timestamp> for TimestampConversion {
    #[inline(always)]
    fn convert_from(value: &Timestamp) -> Self {
        value.as_nanosecond()
    }
}

impl ConvertFrom<TimestampConversion> for Timestamp {
    #[inline(always)]
    fn convert_from(value: TimestampConversion) -> Self {
        Timestamp::from_nanosecond(value).expect("Cant Decode Invalid Timestamp")
    }
}

impl_convert!(Timestamp, TimestampConversion);

#[cfg(test)]
mod tests {
    use jiff::Timestamp;

    #[test]
    fn test() {
        assert!(crate::decode::<Timestamp>(&crate::encode(
            &Timestamp::from_nanosecond(Timestamp::now().as_nanosecond()).unwrap()
        ))
        .is_ok());
        assert!(
            crate::decode::<Timestamp>(&crate::encode(&Timestamp::MIN.as_nanosecond())).is_ok()
        );
        assert!(
            crate::decode::<Timestamp>(&crate::encode(&Timestamp::MAX.as_nanosecond())).is_ok()
        );
    }

    // use alloc::vec::Vec;
    // fn bench_data() -> Vec<Time> {
    //     crate::random_data(1000)
    //         .into_iter()
    //         .map(|(h, m, s, n): (u8, u8, u8, u32)| {
    //             Time::from_hms_nano(h % 24, m % 60, s % 60, n % 1_000_000_000).unwrap()
    //         })
    //         .collect()
    // }
    // crate::bench_encode_decode!(duration_vec: Vec<_>);
}
