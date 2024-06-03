use std::any::type_name;

use num::{FromPrimitive, Num, ToPrimitive};

use crate::unit::YttriaUnitSqrt;

pub trait YttriaVectorStatistics<T> {
    fn min(&self) -> T;
    fn max(&self) -> T;
    fn extremes(&self) -> (T, T);

    fn mean(&self) -> T;
    fn var(&self) -> T;
    fn std(&self) -> T;
}

impl<T> YttriaVectorStatistics<T> for [T]
where
    T: Num
        + YttriaUnitSqrt<T>
        + PartialOrd
        + ToPrimitive
        + FromPrimitive
        + Send
        + Sync
        + Copy
        + Clone,
{
    fn min(&self) -> T {
        let mut min = self[0];

        for i in &self[1..] {
            min = if *i < min { *i } else { min };
        }
        min
    }

    fn max(&self) -> T {
        let mut max = self[0];

        for i in &self[1..] {
            max = if *i > max { *i } else { max };
        }
        max
    }

    fn extremes(&self) -> (T, T) {
        let mut min = self[0];
        let mut max = self[0];

        for i in &self[1..] {
            min = if *i < min { *i } else { min };

            max = if *i > max { *i } else { max };
        }

        (min, max)
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
                sum += ToPrimitive::to_f64(i).unwrap();
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
                sum += detrended * detrended;
            }

            T::from_f64(sum).unwrap_or_else(|| {
                panic!(
                    "Variance is outside of representable range of type {}",
                    type_name::<T>()
                )
            })
        }
    }

    fn std(&self) -> T {
        self.var().sqrt()
    }
}

#[cfg(test)]
mod test {
    use super::YttriaVectorStatistics;

    #[test]
    fn test_mean_if32() {
        let test = [0.0f32, 1.0, 2.0, 3.0, 4.0, 5.0];
        let out = test.mean();
        println!("{out}");
    }
}
