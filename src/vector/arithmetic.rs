use std::any::type_name;

use num::{clamp, traits::Euclid, FromPrimitive, Num};
use rayon::prelude::*;

use crate::unit::RadioUnitSqrt;

pub trait RadioVectorArithmetic<T> {
    fn sum(&self) -> T;

    fn add_into(&self, other: &[T], out: &mut [T]);
    fn add(&self, other: &[T]) -> Vec<T>;
    fn add_inplace(&mut self, other: &[T]) -> &mut Self;

    fn add_const_into(&self, addend: T, out: &mut [T]);
    fn add_const(&self, addend: T) -> Vec<T>;
    fn add_const_inplace(&mut self, addend: T) -> &mut Self;

    fn subtract_into(&self, other: &[T], out: &mut [T]);
    fn subtract(&self, other: &[T]) -> Vec<T>;
    fn subtract_inplace(&mut self, other: &[T]) -> &mut Self;

    fn subtract_const_into(&self, subtrahend: T, out: &mut [T]);
    fn subtract_const(&self, subtrahend: T) -> Vec<T>;
    fn subtract_const_inplace(&mut self, subtrahend: T) -> &mut Self;

    fn multiply_into(&self, other: &[T], out: &mut [T]);
    fn multiply(&self, other: &[T]) -> Vec<T>;
    fn multiply_inplace(&mut self, other: &[T]) -> &mut Self;

    fn multiply_const_into(&self, multiplier: T, out: &mut [T]);
    fn multiply_const(&self, multiplier: T) -> Vec<T>;
    fn multiply_const_inplace(&mut self, multiplier: T) -> &mut Self;

    fn divide_into(&self, other: &[T], out: &mut [T]);
    fn divide(&self, other: &[T]) -> Vec<T>;
    fn divide_inplace(&mut self, other: &[T]) -> &mut Self;

    fn divide_const_into(&self, divisor: T, out: &mut [T]);
    fn divide_const(&self, divisor: T) -> Vec<T>;
    fn divide_const_inplace(&mut self, divisor: T) -> &mut Self;

    fn powi_into(&self, power: u8, out: &mut [T]);
    fn powi(&mut self, power: u8) -> Vec<T>;
    fn powi_inplace(&mut self, power: u8) -> &mut Self;

    fn sqrt_into(&self, out: &mut [T])
    where T: RadioUnitSqrt<T>;
    fn sqrt(&self) -> Vec<T>
    where T: RadioUnitSqrt<T>;
    fn sqrt_inplace(&mut self) -> &mut Self
    where T: RadioUnitSqrt<T>;

    fn diff_into(&self, out: &mut [T]);
    fn diff(&self) -> Vec<T>;
    fn diff_in_place(&mut self) -> &mut Self;

    fn cumsum_into(&self, out: &mut [T]);
    fn cumsum(&self) -> Vec<T>;
    fn cumsum_in_place(&mut self) -> &mut Self;

    fn clamp_into(&self, out: &mut [T], min: T, max: T)
    where
        T: PartialOrd;
    fn clamp(&self, min: T, max: T) -> Vec<T>
    where
        T: PartialOrd;
    fn clamp_in_place(&mut self, min: T, max: T) -> &mut Self
    where
        T: PartialOrd;

    fn convolve_into(&self, out: &[T], out: &mut [T]);
    fn convolve(&self, other: &[T]) -> Vec<T>;

    fn trapz(&self) -> T;

    fn interp_into(&self, out: &mut [T], xp: &[T], fp: &[T])
    where
        T: PartialOrd;
    fn interp(&self, xp: &[T], fp: &[T]) -> Vec<T>
    where
        T: PartialOrd;
    fn interp_in_place(&mut self, xp: &[T], fp: &[T]) -> &mut Self
    where
        T: PartialOrd;

    fn angle_unwrap_into(&self, out: &mut [T], period: Option<T>)
    where
        T: FromPrimitive + Euclid;
    fn angle_unwrap(&self, period: Option<T>) -> Vec<T>
    where
        T: FromPrimitive + Euclid;
    fn angle_unwrap_in_place(&mut self, period: Option<T>) -> &mut Self
    where
        T: FromPrimitive + Euclid;
}

impl<T> RadioVectorArithmetic<T> for [T]
where
    T: Num + Send + Sync + Copy,
{
    fn sum(&self) -> T {
        let mut accumulator = T::zero();
        for i in self {
            accumulator = accumulator + *i;
        }
        accumulator
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
    fn add_inplace(&mut self, other: &[T]) -> &mut Self {
        self.par_iter_mut().zip(other).for_each(|(out, other)| {
            *out = *out + *other;
        });
        self
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
    fn add_const_inplace(&mut self, addend: T) -> &mut Self {
        self.par_iter_mut().for_each(|out| {
            *out = *out + addend;
        });
        self
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
    fn subtract_inplace(&mut self, other: &[T]) -> &mut Self {
        self.par_iter_mut().zip(other).for_each(|(out, other)| {
            *out = *out - *other;
        });
        self
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

    fn subtract_const_inplace(&mut self, subtrahend: T) -> &mut Self {
        self.par_iter_mut().for_each(|out| {
            *out = *out - subtrahend;
        });
        self
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
    fn multiply_inplace(&mut self, other: &[T]) -> &mut Self {
        self.par_iter_mut().zip(other).for_each(|(out, other)| {
            *out = *out * *other;
        });
        self
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
    fn multiply_const_inplace(&mut self, multiplier: T) -> &mut Self {
        self.par_iter_mut().for_each(|out| {
            *out = *out * multiplier;
        });
        self
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
    fn divide_inplace(&mut self, other: &[T]) -> &mut Self {
        self.par_iter_mut().zip(other).for_each(|(out, other)| {
            *out = *out / *other;
        });
        self
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
    fn divide_const_inplace(&mut self, divisor: T) -> &mut Self {
        self.par_iter_mut().for_each(|out| {
            *out = *out / divisor;
        });
        self
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

    fn powi_inplace(&mut self, power: u8) -> &mut Self {
        self.par_iter_mut().for_each(|own| {
            let base = *own;
            *own = T::one();
            for _ in 0..power {
                *own = *own * base;
            }
        });
        self
    }

    fn sqrt_into(&self, out: &mut [T])
    where T: RadioUnitSqrt<T>
    {
        out.par_iter_mut().zip(self).for_each(|(out, own)| {
            *out = own.sqrt();
        });
    }

    fn sqrt(&self) -> Vec<T>
    where T: RadioUnitSqrt<T>
    {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) }
        self.sqrt_into(&mut out);
        out
    }

    fn sqrt_inplace(&mut self) -> &mut Self
    where T: RadioUnitSqrt<T>
    {
        self.par_iter_mut().for_each(|own| {
            *own = own.sqrt();
        });
        self
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

    fn diff_in_place(&mut self) -> &mut Self {
        for i in (1..(self.len())).rev() {
            self[i] = self[i] - self[i - 1]
        }
        self
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

    fn cumsum_in_place(&mut self) -> &mut Self {
        let mut sum = T::zero();
        for out in self.iter_mut() {
            sum = sum + *out;
            *out = sum;
        }
        self
    }

    fn clamp_into(&self, out: &mut [T], min: T, max: T)
    where
        T: PartialOrd,
    {
        out.par_iter_mut()
            .zip(self)
            .for_each(|(out, own)| *out = clamp(*own, min, max));
    }

    fn clamp(&self, min: T, max: T) -> Vec<T>
    where
        T: PartialOrd,
    {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) }
        self.clamp_into(out.as_mut_slice(), min, max);
        out
    }

    fn clamp_in_place(&mut self, min: T, max: T) -> &mut Self
    where
        T: PartialOrd,
    {
        self.par_iter_mut().for_each(|own| {
            *own = clamp(*own, min, max);
        });
        self
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

    fn trapz(&self) -> T {
        let mut out = T::zero();
        let two = T::one() + T::one();

        for (a, b) in self.iter().zip(&self[1..]) {
            out = out + (*a * *b) / two;
        }

        out
    }

    fn interp_into(&self, out: &mut [T], xp: &[T], fp: &[T])
    where
        T: PartialOrd,
    {
        out.par_iter_mut().zip(self).for_each(|(out, own)| {
            let bin = xp.iter().position(|&pos| pos >= *own).unwrap_or(xp.len());
            if bin == 0 {
                *out = fp[0];
            } else if bin == xp.len() {
                *out = fp[fp.len() - 1];
            } else {
                let slope = (fp[bin] - fp[bin - 1]) / (xp[bin] - xp[bin - 1]);
                *out = fp[bin - 1] + slope * (*own - xp[bin - 1])
            }
        });
    }

    fn interp(&self, xp: &[T], fp: &[T]) -> Vec<T>
    where
        T: PartialOrd,
    {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) }
        self.interp_into(&mut out, xp, fp);
        out
    }

    fn interp_in_place(&mut self, xp: &[T], fp: &[T]) -> &mut Self
    where
        T: PartialOrd,
    {
        self.par_iter_mut().for_each(|out| {
            let bin = xp.iter().position(|&pos| pos >= *out).unwrap_or(xp.len());
            if bin == 0 {
                *out = fp[0];
            } else if bin == xp.len() {
                *out = fp[fp.len() - 1];
            } else {
                let slope = (fp[bin] - fp[bin - 1]) / (xp[bin] - xp[bin - 1]);
                *out = fp[bin - 1] + slope * (*out - xp[bin - 1])
            }
        });
        self
    }

    fn angle_unwrap_into(&self, out: &mut [T], period: Option<T>)
    where
        T: FromPrimitive + Euclid,
    {
        let period = period.unwrap_or_else(|| {
            T::from_f64(2.0 * std::f64::consts::PI).expect(
                format!("Could not convert 2 * pi into type: '{}'", type_name::<T>()).as_str(),
            )
        });
        let discont = period / T::from_u8(2).unwrap();
        for idx in 1..(out.len()) {
            let diff = self[idx] - self[idx - 1];
            let wrapped_diff = (diff + discont).rem_euclid(&period) - discont;
            out[idx] = out[idx - 1] + wrapped_diff;
        }
    }

    fn angle_unwrap(&self, period: Option<T>) -> Vec<T>
    where
        T: FromPrimitive + Euclid,
    {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) }
        out[0] = T::zero();
        self.angle_unwrap_into(&mut out, period);
        out
    }

    fn angle_unwrap_in_place(&mut self, period: Option<T>) -> &mut Self
    where
        T: FromPrimitive + Euclid,
    {
        let period = period.unwrap_or_else(|| {
            T::from_f64(2.0 * std::f64::consts::PI).expect(
                format!("Could not convert 2 * pi into type: '{}'", type_name::<T>()).as_str(),
            )
        });
        let discont = period / T::from_u8(2).unwrap();
        for idx in 1..(self.len()) {
            let diff = self[idx] - self[idx - 1];
            let wrapped_diff = (diff + discont).rem_euclid(&period) - discont;
            self[idx] = self[idx - 1] + wrapped_diff;
        }
        self
    }
}

#[cfg(test)]
mod test {
    use super::RadioVectorArithmetic;

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

    #[test]
    fn test_interp_f32() {
        let test = [-1.0, -0.5, 0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];
        let x = [0.0, 1.0, 2.0];
        let y = [0.0, 1.0, 0.0];
        let interpd = test.interp(&x, &y);
        println!("{interpd:?}");
    }
}
