use std::mem::size_of;

use num::{FromPrimitive, Integer};

pub trait YttriaVectorBitwise {
    fn packbits(&self) -> Vec<u8>;
    fn unpackbits(&self) -> Vec<u8>;
    fn pack_into<T>(&self) -> T
    where
        T: Integer + FromPrimitive + std::ops::Shl<Output = T> + std::ops::BitOr<Output = T>;
}

impl YttriaVectorBitwise for [u8] {
    fn packbits(&self) -> Vec<u8> {
        self.chunks(8)
            .map(|x| {
                let mut out = 0u8;
                let mut offset = 7;
                for i in x {
                    out |= *i << offset;
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
                    x & 0x1,
                ]
            })
            .collect::<Vec<_>>()
    }

    fn pack_into<T>(&self) -> T
    where
        T: Integer + FromPrimitive + std::ops::Shl<Output = T> + std::ops::BitOr<Output = T>,
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
    use super::YttriaVectorBitwise;

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
        let data = [1u8, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1];
        let expected_packed = 33_039u16;

        let packed: u16 = data.pack_into();
        println!("{packed:b}");

        assert!(packed == expected_packed);

        let recon_data = &packed.to_be_bytes().unpackbits();
        println!("{recon_data:?}");

        assert!(data.iter().eq(recon_data.iter()));
    }
}
