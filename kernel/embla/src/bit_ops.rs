use core::ops::{Bound, RangeBounds};

pub trait BitOps {
    const BIT_WIDTH: usize;

    fn bit(&self, bit: usize) -> bool;
    fn bits(&self, bits: impl RangeBounds<usize>) -> Self;

    fn set_bit(&mut self, bit: usize, value: bool) -> &mut Self;
    fn set_bits(&mut self, bits: impl RangeBounds<usize>, value: Self) -> &mut Self;
}

macro_rules! impl_bitops {
    ($t:ty) => {
        impl BitOps for $t {
            const BIT_WIDTH: usize = core::mem::size_of::<Self>() * 8;

            #[inline]
            #[track_caller]
            fn bit(&self, bit: usize) -> bool {
                assert!(bit < Self::BIT_WIDTH);
                (*self & (1 << bit)) != 0
            }

            #[inline]
            #[track_caller]
            fn bits(&self, bits: impl RangeBounds<usize>) -> Self {
                let (start, end) = extract_bounds::<Self>(bits);
                assert!(start < Self::BIT_WIDTH);
                assert!(end <= Self::BIT_WIDTH);
                assert!(start <= end);

                if start == end {
                    0
                } else {
                    (*self << (Self::BIT_WIDTH - end) >> (Self::BIT_WIDTH - end)) >> start
                }
            }

            #[inline]
            #[track_caller]
            fn set_bit(&mut self, bit: usize, value: bool) -> &mut Self {
                assert!(bit < Self::BIT_WIDTH);

                if value {
                    *self |= 1 << bit;
                } else {
                    *self &= !(1 << bit);
                }

                self
            }

            #[inline]
            #[track_caller]
            fn set_bits(&mut self, bits: impl RangeBounds<usize>, value: Self) -> &mut Self {
                let (start, end) = extract_bounds::<Self>(bits);

                assert!(start < Self::BIT_WIDTH);
                assert!(end <= Self::BIT_WIDTH);
                assert!(start <= end);
                assert!(
                    start == end && value == 0
                        || (value << (Self::BIT_WIDTH - (end - start))
                            >> (Self::BIT_WIDTH - (end - start)))
                            == value,
                    "value does not fit into specified bit range!"
                );

                if start != end {
                    let mask = !(!0 << (Self::BIT_WIDTH - end) >> (Self::BIT_WIDTH - end) >> start
                        << start);
                    *self = (*self & mask) | (value << start);
                }

                self
            }
        }
    };
}

impl_bitops!(u8);
impl_bitops!(u16);
impl_bitops!(u32);
impl_bitops!(u64);
impl_bitops!(u128);
impl_bitops!(usize);
impl_bitops!(i8);
impl_bitops!(i16);
impl_bitops!(i32);
impl_bitops!(i64);
impl_bitops!(i128);
impl_bitops!(isize);

#[inline]
fn extract_bounds<T: BitOps>(bits: impl RangeBounds<usize>) -> (usize, usize) {
    let start = match bits.start_bound() {
        Bound::Included(&value) => value,
        Bound::Excluded(&value) => value + 1,
        Bound::Unbounded => 0,
    };
    let end = match bits.end_bound() {
        Bound::Included(&value) => value + 1,
        Bound::Excluded(&value) => value,
        Bound::Unbounded => T::BIT_WIDTH,
    };
    (start, end)
}
