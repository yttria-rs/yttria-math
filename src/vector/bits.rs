use super::integer::DspInt;

use std::mem::size_of;

pub trait DspBitwise<T>:
    DspInt
    + std::ops::BitOr<Output = T>
    + std::ops::BitAnd<Output = T>
    + std::ops::BitXor<Output = T>
    + std::ops::Shl<Output = T>
    + std::ops::Shr<Output = T>
{
}
impl<T> DspBitwise<T> for T where
    T: DspInt
        + std::ops::BitOr<Output = T>
        + std::ops::BitAnd<Output = T>
        + std::ops::BitXor<Output = T>
        + std::ops::Shl<Output = T>
        + std::ops::Shr<Output = T>
{
}

pub trait BitMath {
    fn packbits(&self) -> Vec<u8>;
    fn unpackbits(&self) -> Vec<u8>;
    fn pack_into<T>(&self) -> T
    where
        T: DspBitwise<T> + std::ops::Shl<usize, Output = T> + std::ops::BitOr<Output = T>;
}

impl BitMath for [u8] {
    fn packbits(&self) -> Vec<u8> {
        self.chunks(8)
            .map(|x| {
                let mut out = 0u8;
                let mut offset = 7;
                for i in x {
                    out |= (*i as u8) << offset;
                    offset -= 1;
                }
                out
            })
            .collect::<Vec<_>>()
    }

    fn unpackbits(&self) -> Vec<u8> {
        self.iter()
            .flat_map(|x| {
                [
                    (x >> 7) & 0x1,
                    (x >> 6) & 0x1,
                    (x >> 5) & 0x1,
                    (x >> 4) & 0x1,
                    (x >> 3) & 0x1,
                    (x >> 2) & 0x1,
                    (x >> 1) & 0x1,
                    (x >> 0) & 0x1,
                ]
            })
            .collect::<Vec<_>>()
    }

    fn pack_into<T>(&self) -> T
    where
        T: DspBitwise<T>,
    {
        assert!(self.len() <= size_of::<T>() * 8);

        let mut sum = T::zero();

        for (idx, i) in self.iter().enumerate() {
            let data_bit = T::from_u8(*i).expect("");
            let shift = T::from_usize(self.len() - 1 - idx).expect("");
            sum = sum | (data_bit << shift);
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::BitMath;

    #[test]
    fn test_unpack_bits() {
        let data = [129u8, 15];
        let expected_unpacked = [1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1];
        let bits = data.unpackbits();
        println!("{bits:?}");

        assert!(bits.iter().eq(expected_unpacked.iter()));

        let recon_data = bits.packbits();
        println!("{recon_data:?}");

        assert!(data.iter().eq(recon_data.iter()));
    }

    #[test]
    fn test_pack_into_u64() {
        let data_unpacked = [1u8, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1];
        // let expected_packed = [129u8, 15];

        let packed: u32 = data_unpacked.pack_into();
        println!("{packed:b}");

        // assert!(bits.iter().eq(expected_unpacked.iter()));

        // let recon_data = bits.packbits();
        // println!("{recon_data:?}");

        // assert!(data.iter().eq(recon_data.iter()));
    }
}
