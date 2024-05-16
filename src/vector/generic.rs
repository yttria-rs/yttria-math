use num::{FromPrimitive, Num, NumCast, ToPrimitive};
use rayon::prelude::*;
use std::any::type_name;

pub trait DspGeneric:
    Num + FromPrimitive + ToPrimitive + std::fmt::Debug + Send + Sync + Copy + Clone + Sized
{
}
impl<T> DspGeneric for T where
    T: Num + FromPrimitive + ToPrimitive + std::fmt::Debug + Send + Sync + Copy
{
}

pub trait GenericVectorMath<T> {
    fn sum(&self) -> T;
    fn mean(&self) -> T;
    fn var(&self) -> T;

    fn add_into(&self, other: &[T], out: &mut [T]);
    fn add(&self, other: &[T]) -> Vec<T>;
    fn add_inplace(&mut self, other: &[T]);

    fn add_const_into(&self, addend: T, out: &mut [T]);
    fn add_const(&self, addend: T) -> Vec<T>;
    fn add_const_inplace(&mut self, addend: T);

    fn subtract_into(&self, other: &[T], out: &mut [T]);
    fn subtract(&self, other: &[T]) -> Vec<T>;
    fn subtract_inplace(&mut self, other: &[T]);

    fn subtract_const_into(&self, subtrahend: T, out: &mut [T]);
    fn subtract_const(&self, subtrahend: T) -> Vec<T>;
    fn subtract_const_inplace(&mut self, subtrahend: T);

    fn multiply_into(&self, other: &[T], out: &mut [T]);
    fn multiply(&self, other: &[T]) -> Vec<T>;
    fn multiply_inplace(&mut self, other: &[T]);

    fn multiply_const_into(&self, multiplier: T, out: &mut [T]);
    fn multiply_const(&self, multiplier: T) -> Vec<T>;
    fn multiply_const_inplace(&mut self, multiplier: T);

    fn divide_into(&self, other: &[T], out: &mut [T]);
    fn divide(&self, other: &[T]) -> Vec<T>;
    fn divide_inplace(&mut self, other: &[T]);

    fn divide_const_into(&self, divisor: T, out: &mut [T]);
    fn divide_const(&self, divisor: T) -> Vec<T>;
    fn divide_const_inplace(&mut self, divisor: T);

    fn powi_into(&self, power: u8, out: &mut [T]);
    fn powi(&mut self, power: u8) -> Vec<T>;
    fn powi_inplace(&mut self, power: u8);

    fn diff_into(&self, out: &mut [T]);
    fn diff(&self) -> Vec<T>;
    fn diff_in_place(&mut self);

    fn cumsum_into(&self, out: &mut [T]);
    fn cumsum(&self) -> Vec<T>;
    fn cumsum_in_place(&mut self);

    fn convolve_into(&self, out: &[T], out: &mut [T]);
    fn convolve(&self, other: &[T]) -> Vec<T>;

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

impl<T: DspGeneric> GenericVectorMath<T> for [T] {
    fn sum(&self) -> T {
        let mut accumulator = T::zero();
        for i in self {
            accumulator = accumulator + *i;
        }
        accumulator
    }

    fn mean(&self) -> T {
        if let Some(size) = T::from_usize(self.len()) {
            let mut sum = T::zero();
            for i in self {
                sum = sum + *i;
            }
            sum / size
        }
        // fallback in case there are more elements in the slice than the type can support.
        // This should mostly work identically except for some potential edge cases, and is less
        // efficient than the normal implementation, hence the reason for this as a fallback.
        else {
            let mut sum = 0.0f64;
            let size = self.len() as f64;

            for i in self {
                sum = sum + ToPrimitive::to_f64(i).unwrap();
            }

            sum /= size;

            T::from_f64(sum).unwrap()
        }
    }

    fn var(&self) -> T {
        if let Some(size) = T::from_usize(self.len()) {
            let mut sum = T::zero();
            let mean = self.mean();
            for i in self {
                let detrended = *i - mean;
                sum = sum + detrended * detrended;
            }

            sum / size
        } else {
            let mut sum = 0.0f64;
            let mean = ToPrimitive::to_f64(&self.mean()).unwrap();
            for i in self {
                let detrended = ToPrimitive::to_f64(i).unwrap() - mean;
                sum = sum + detrended * detrended;
            }

            T::from_f64(sum).expect(
                format!(
                    "Variance is outside of representable range of type {}",
                    type_name::<T>()
                )
                .as_str(),
            )
        }
    }

    fn add_into(&self, other: &[T], out: &mut [T]) {
        out.par_iter_mut()
            .zip(self)
            .zip(other)
            .for_each(|((out, own), other)| {
                *out = *own + *other;
            });
    }
    fn add(&self, other: &[T]) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) };
        self.add_into(other, out.as_mut_slice());
        out
    }
    fn add_inplace(&mut self, other: &[T]) {
        self.par_iter_mut().zip(other).for_each(|(out, other)| {
            *out = *out + *other;
        });
    }

    fn add_const_into(&self, addend: T, out: &mut [T]) {
        out.par_iter_mut().zip(self).for_each(|(out, own)| {
            *out = *own + addend;
        });
    }

    fn add_const(&self, addend: T) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) };
        self.add_const_into(addend, out.as_mut_slice());
        out
    }
    fn add_const_inplace(&mut self, addend: T) {
        self.par_iter_mut().for_each(|out| {
            *out = *out + addend;
        });
    }

    fn subtract_into(&self, other: &[T], out: &mut [T]) {
        out.par_iter_mut()
            .zip(self)
            .zip(other)
            .for_each(|((out, own), other)| {
                *out = *own - *other;
            });
    }
    fn subtract(&self, other: &[T]) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) };
        self.subtract_into(other, out.as_mut_slice());
        out
    }
    fn subtract_inplace(&mut self, other: &[T]) {
        self.par_iter_mut().zip(other).for_each(|(out, other)| {
            *out = *out - *other;
        });
    }

    fn subtract_const_into(&self, subtrahend: T, out: &mut [T]) {
        out.par_iter_mut().zip(self).for_each(|(out, own)| {
            *out = *own - subtrahend;
        });
    }

    fn subtract_const(&self, subtrahend: T) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) };
        self.subtract_const_into(subtrahend, out.as_mut_slice());
        out
    }

    fn subtract_const_inplace(&mut self, subtrahend: T) {
        self.par_iter_mut().for_each(|out| {
            *out = *out - subtrahend;
        });
    }

    fn multiply_into(&self, other: &[T], out: &mut [T]) {
        out.par_iter_mut()
            .zip(self)
            .zip(other)
            .for_each(|((out, own), other)| {
                *out = *own * *other;
            });
    }
    fn multiply(&self, other: &[T]) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) };
        self.multiply_into(other, out.as_mut_slice());
        out
    }
    fn multiply_inplace(&mut self, other: &[T]) {
        self.par_iter_mut().zip(other).for_each(|(out, other)| {
            *out = *out * *other;
        });
    }

    fn multiply_const_into(&self, multiplier: T, out: &mut [T]) {
        out.par_iter_mut().zip(self).for_each(|(out, own)| {
            *out = *own * multiplier;
        });
    }
    fn multiply_const(&self, multiplier: T) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) };
        self.multiply_const_into(multiplier, out.as_mut_slice());
        out
    }
    fn multiply_const_inplace(&mut self, multiplier: T) {
        self.par_iter_mut().for_each(|out| {
            *out = *out * multiplier;
        });
    }

    fn divide_into(&self, other: &[T], out: &mut [T]) {
        out.par_iter_mut()
            .zip(self)
            .zip(other)
            .for_each(|((out, own), other)| {
                *out = *own / *other;
            });
    }
    fn divide(&self, other: &[T]) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) };
        self.divide_into(other, out.as_mut_slice());
        out
    }
    fn divide_inplace(&mut self, other: &[T]) {
        self.par_iter_mut().zip(other).for_each(|(out, other)| {
            *out = *out / *other;
        });
    }

    fn divide_const_into(&self, divisor: T, out: &mut [T]) {
        out.par_iter_mut().zip(self).for_each(|(out, own)| {
            *out = *own / divisor;
        });
    }
    fn divide_const(&self, divisor: T) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) };
        self.divide_const_into(divisor, out.as_mut_slice());
        out
    }
    fn divide_const_inplace(&mut self, divisor: T) {
        self.par_iter_mut().for_each(|out| {
            *out = *out / divisor;
        });
    }

    fn powi_into(&self, power: u8, out: &mut [T]) {
        out.par_iter_mut().zip(self).for_each(|(out, own)| {
            *out = T::one();
            for _ in 0..power {
                *out = *out * *own;
            }
        });
    }

    fn powi(&mut self, power: u8) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) };
        self.powi_into(power, out.as_mut_slice());
        out
    }

    fn powi_inplace(&mut self, power: u8) {
        self.par_iter_mut().for_each(|own| {
            let base = *own;
            *own = T::one();
            for _ in 0..power {
                *own = *own * base;
            }
        });
    }

    fn diff_into(&self, out: &mut [T]) {
        out.par_iter_mut().enumerate().for_each(|(idx, out)| {
            *out = self[idx + 1] - self[idx];
        });
    }

    fn diff(&self) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len() - 1);
        unsafe { out.set_len(self.len() - 1) };
        self.diff_into(out.as_mut_slice());
        out
    }

    fn diff_in_place(&mut self) {
        for i in (1..(self.len())).rev() {
            self[i] = self[i] - self[i - 1]
        }
    }

    fn cumsum_into(&self, out: &mut [T]) {
        let mut sum = T::zero();
        for (out, next) in out.iter_mut().zip(self) {
            sum = sum + *next;
            *out = sum;
        }
    }

    fn cumsum(&self) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) }
        self.cumsum_into(&mut out);
        out
    }

    fn cumsum_in_place(&mut self) {
        let mut sum = T::zero();
        for out in self.iter_mut() {
            sum = sum + *out;
            *out = sum;
        }
    }

    fn convolve_into(&self, other: &[T], out: &mut [T]) {
        out.par_iter_mut().enumerate().for_each(|(idx_out, out)| {
            let lower_bound = 0isize.max(idx_out as isize + 1 - self.len() as isize) as usize;
            let upper_bound = other.len().min(idx_out);
            for idx_n in lower_bound..upper_bound {
                *out = *out + self[idx_out - idx_n] * other[idx_n];
            }
        });
    }

    fn convolve(&self, other: &[T]) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) }
        self.convolve_into(other, &mut out);
        out
    }

    fn repeat(&self, repeats: usize) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len() * repeats);
        unsafe { out.set_len(self.len() * repeats) };

        out.par_iter_mut()
            .enumerate()
            .for_each(|(idx, x)| *x = self[idx / repeats]);

        out
    }

    fn tile(&self, repeats: usize) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len() * repeats);
        unsafe { out.set_len(self.len() * repeats) };

        out.par_iter_mut()
            .enumerate()
            .for_each(|(idx, x)| *x = self[idx % self.len()]);

        out
    }

    fn concatenate(&self, other: &[T]) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len() + other.len());
        unsafe { out.set_len(self.len() + other.len()) };

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
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) }
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
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) };

        out.par_iter_mut().zip(self).for_each(|(out, own)| {
            *out = U::from(*own).expect(format!("Could not cast type '{}' to '{}'", type_name::<T>(), type_name::<U>()).as_str());
        });

        out
    }
}

#[cfg(test)]
mod test {
    use super::GenericVectorMath;

    #[test]
    fn test_mean_i32() {
        let test = [0i32, 1, 2, 3, 4, 5];
        let out = test.mean();
        println!("{out}");
    }

    #[test]
    fn test_add_i32() {
        let one = [0i32, 1, 2, 3, 4, 5];
        let two = [0i32, 1, -1, 1, -1, 1];

        let out = one.add(two.as_slice());
        println!("{out:?}");

        let out = out.add_const(2);
        println!("{out:?}");
    }

    #[test]
    fn test_diff_i32() {
        let test = [0i32, 1, 5, 11];
        let interpd = test.diff();
        println!("{interpd:?}");
    }

    #[test]
    fn test_diff_f32() {
        let test = [0.0f32, 1.0, 5.0, 11.0];
        let interpd = test.diff();
        println!("{interpd:?}");
    }

    // #[test]
    // fn test_interp_i32() {
    //     let test = [0i32, 1, 5, 11];
    //     let x = [0, 1, 2, 3, 4];
    //     let y = [0, 1, 0, 2, 0];
    //     let interpd = test.interp(&x, &y);
    //     println!("{interpd:?}");
    // }

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
