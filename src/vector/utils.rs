use num::{Num, NumCast, ToPrimitive};
use rayon::prelude::*;
use std::any::type_name;

pub trait YttriaVectorUtils<T> {
    fn repeat(&self, repeats: usize) -> Vec<T>;
    fn tile(&self, repeats: usize) -> Vec<T>;
    fn concatenate(&self, other: &[T]) -> Vec<T>;

    fn roll_into(&self, out: &mut [T], shift: usize);
    fn roll(&self, shift: usize) -> Vec<T>;
    fn roll_in_place(&mut self, shift: usize);

    fn fftshift_into(&self, out: &mut [T]);
    fn fftshift(&self) -> Vec<T>;
    fn fftshift_in_place(&mut self);

    fn as_type<U: NumCast + Send + Sync>(&self) -> Vec<U>;
}

impl<T> YttriaVectorUtils<T> for [T]
where
    T: Num + ToPrimitive + Send + Sync + Copy + Clone,
{
    fn repeat(&self, repeats: usize) -> Vec<T> {
        let mut out = vec![T::zero(); self.len() * repeats];

        out.par_iter_mut()
            .enumerate()
            .for_each(|(idx, x)| *x = self[idx / repeats]);

        out
    }

    fn tile(&self, repeats: usize) -> Vec<T> {
        let mut out = vec![T::zero(); self.len() * repeats];

        out.par_iter_mut()
            .enumerate()
            .for_each(|(idx, x)| *x = self[idx % self.len()]);

        out
    }

    fn concatenate(&self, other: &[T]) -> Vec<T> {
        let mut out = vec![T::zero(); self.len() + other.len()];

        out[..(self.len())].copy_from_slice(self);
        out[(self.len())..].copy_from_slice(other);

        out
    }

    fn roll_into(&self, other: &mut [T], shift: usize) {
        other.par_iter_mut().enumerate().for_each(|(idx, out)| {
            *out = self[(idx + shift) % self.len()];
        });
    }

    fn roll(&self, shift: usize) -> Vec<T> {
        let mut out = vec![T::zero(); self.len()];
        self.roll_into(out.as_mut_slice(), shift);
        out
    }

    fn roll_in_place(&mut self, shift: usize) {
        for idx in 0..(self.len()) {
            self[idx] = self[(idx + shift) % self.len()];
        }
    }

    fn fftshift_into(&self, other: &mut [T]) {
        self.roll_into(other, self.len() / 2);
    }

    fn fftshift(&self) -> Vec<T> {
        self.roll(self.len() / 2)
    }

    fn fftshift_in_place(&mut self) {
        self.roll_in_place(self.len() / 2);
    }

    fn as_type<U: NumCast + Send + Sync>(&self) -> Vec<U> {
        self.par_iter()
            .map(|&value| {
                U::from(value).unwrap_or_else(|| {
                    panic!(
                        "Could not cast type '{}' to '{}'",
                        type_name::<T>(),
                        type_name::<U>()
                    )
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::YttriaVectorUtils;

    #[test]
    fn test_fftshift() {
        let freqs = [0., 1., 2., 3., 4., -5., -4., -3., -2., -1.];
        let shifted = freqs.fftshift();
        println!("{shifted:?}");
    }

    #[test]
    fn test_u8_as_f32() {
        let test = [0u8, 5, 16, 32];
        let cast = test.as_type::<f32>();

        println!("{cast:?}");
    }
}
