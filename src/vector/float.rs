use num::{traits::Euclid, Float};
use rayon::prelude::*;
use std::any::type_name;

use super::generic::{DspGeneric, GenericVectorMath};
use crate::utils::partial_clamp;

pub trait DspFloat: DspGeneric + Float {}
impl<T> DspFloat for T where T: DspGeneric + Float {}

pub trait FloatVectorMath<T> {
    fn min(&self) -> T;
    fn max(&self) -> T;
    fn extremes(&self) -> (T, T);

    fn sqrt_into(&self, out: &mut [T]);
    fn sqrt(&self) -> Vec<T>;
    fn sqrt_inplace(&mut self);

    fn std(&self) -> T;
    fn trapz(&self) -> T;

    fn interp_into(&self, out: &mut [T], xp: &[T], fp: &[T]);
    fn interp(&self, xp: &[T], fp: &[T]) -> Vec<T>;
    fn interp_in_place(&mut self, xp: &[T], fp: &[T]);

    fn clamp_into(&self, out: &mut [T], min: T, max: T);
    fn clamp(&self, min: T, max: T) -> Vec<T>;
    fn clamp_in_place(&mut self, min: T, max: T);

    fn angle_unwrap_into(&self, out: &mut [T], period: Option<T>);
    fn angle_unwrap(&self, period: Option<T>) -> Vec<T>;
    fn angle_unwrap_in_place(&mut self, period: Option<T>);
}

impl<T> FloatVectorMath<T> for [T]
where
    T: DspFloat + Euclid,
{
    fn min(&self) -> T {
        let mut min = self[0];

        for i in &self[1..] {
            min = min.min(*i);
        }
        min
    }

    fn max(&self) -> T {
        let mut max = self[0];

        for i in &self[1..] {
            max = max.max(*i);
        }
        max
    }

    fn extremes(&self) -> (T, T) {
        let mut min = self[0];
        let mut max = self[0];

        for i in &self[1..] {
            min = min.min(*i);

            max = max.max(*i);
        }

        (min, max)
    }

    fn sqrt_into(&self, out: &mut [T]) {
        out.par_iter_mut().zip(self).for_each(|(out, own)| {
            *out = own.sqrt();
        });
    }

    fn sqrt(&self) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) }
        self.sqrt_into(&mut out);
        out
    }

    fn sqrt_inplace(&mut self) {
        self.par_iter_mut().for_each(|own| {
            *own = own.sqrt();
        });
    }

    fn std(&self) -> T {
        self.var().sqrt()
    }

    fn trapz(&self) -> T {
        let mut out = T::zero();
        let two = T::from_u8(2).unwrap();

        for (a, b) in self.iter().zip(&self[1..]) {
            out = out + (*a * *b) / two;
        }

        out
    }

    fn interp_into(&self, out: &mut [T], xp: &[T], fp: &[T]) {
        out.par_iter_mut().zip(self).for_each(|(out, own)| {
            let bin = xp
                .iter()
                .position(|&pos| pos >= *own)
                .unwrap_or(xp.len());
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

    fn interp(&self, xp: &[T], fp: &[T]) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) }
        self.interp_into(&mut out, xp, fp);
        out
    }

    fn interp_in_place(&mut self, xp: &[T], fp: &[T]) {
        self.par_iter_mut().for_each(|out| {
            let bin = xp
                .iter()
                .position(|&pos| pos >= *out)
                .unwrap_or(xp.len());
            if bin == 0 {
                *out = fp[0];
            } else if bin == xp.len() {
                *out = fp[fp.len() - 1];
            } else {
                let slope = (fp[bin] - fp[bin - 1]) / (xp[bin] - xp[bin - 1]);
                *out = fp[bin - 1] + slope * (*out - xp[bin - 1])
            }
        });
        todo!()
    }

    fn clamp_into(&self, out: &mut [T], min: T, max: T) {
        out.par_iter_mut().zip(self).for_each(|(out, own)| {
            *out = partial_clamp(*own, min, max);
        });
    }

    fn clamp(&self, min: T, max: T) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) }
        self.clamp_into(out.as_mut_slice(), min, max);
        out
    }

    fn clamp_in_place(&mut self, min: T, max: T) {
        self.par_iter_mut().for_each(|own| {
            *own = partial_clamp(*own, min, max);
        });
    }

    fn angle_unwrap_into(&self, out: &mut [T], period: Option<T>) {
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

    fn angle_unwrap(&self, period: Option<T>) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) }
        out[0] = T::zero();
        self.angle_unwrap_into(&mut out, period);
        out
    }

    fn angle_unwrap_in_place(&mut self, period: Option<T>) {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interp_f32() {
        let test = [-1.0, -0.5, 0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];
        let x = [0.0, 1.0, 2.0];
        let y = [0.0, 1.0, 0.0];
        let interpd = test.interp(&x, &y);
        println!("{interpd:?}");
    }
}
