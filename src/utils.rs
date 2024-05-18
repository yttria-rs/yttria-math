use num::Complex;
use num::{cast::FromPrimitive, Num};
use std::any::type_name;

use crate::prelude::*;
use crate::windows;

pub fn map<T: Num + Copy>(value: T, from_low: T, from_high: T, to_low: T, to_high: T) -> T {
    (value - from_low) * ((to_high - to_low) / (from_high - from_low)) + to_low
}

pub fn linspace<T: Num + FromPrimitive + Copy>(
    start: T,
    stop: T,
    size: usize,
    endpoint: bool,
) -> Vec<T> {
    let mut out = Vec::with_capacity(size);
    unsafe { out.set_len(size) };

    let delta = if endpoint {
        (stop - start)
            / T::from_usize(size - 1).expect(
                format!(
                    "Could not convert usize '{size}' into type: {}",
                    type_name::<T>()
                )
                .as_str(),
            )
    } else {
        (stop - start)
            / T::from_usize(size).expect(
                format!(
                    "Could not convert usize '{size}' into type: {}",
                    type_name::<T>()
                )
                .as_str(),
            )
    };

    for i in 0..size {
        out[i] = start + delta * T::from_usize(i).unwrap();
    }

    out
}

pub fn arange<T: Num + PartialOrd + Copy>(start: T, stop: T, step: T) -> Vec<T> {
    let mut out = Vec::new();
    let mut curr = start;

    while curr < stop {
        out.push(curr);
        curr = curr + step;
    }

    out
}

pub fn firwin2(numtaps: usize, freqs: &[f64], gains: &[f64], antisymmetric: bool) -> Vec<f64> {
    let mut freqs = freqs.to_vec();

    let nyq = 1.0;
    let nfreqs = 1 + usize::pow(2, f32::log2(numtaps as f32).ceil() as u32);

    // These are sanity checks. I'm sane, so I can skip them for now... Right?
    // let d = freqs.diff();
    // let d2 = d
    //     .iter()
    //     .zip(d[1..].iter())
    //     .map(|(one, two)| *one + *two)
    //     .collect::<Vec<_>>();

    for i in 0..(freqs.len() - 1) {
        if freqs[i] == freqs[i + 1] {
            freqs[i] = freqs[i] - f64::EPSILON;
            freqs[i + 1] = freqs[i + 1] + f64::EPSILON;
        }
    }

    let ftype = match (antisymmetric, numtaps % 2 == 0) {
        (false, false) => 1,
        (false, true) => {
            assert!(
                gains[gains.len() - 1] == 0.0f64,
                "A Type II filter must have zero gain at the Nyquist frequency."
            );
            2
        }
        (true, false) => {
            assert!(
                gains[0] == 0.0f64 && gains[gains.len() - 1] == 0.0f64,
                "A Type III filter must have zero gain at zero and Nyquist frequencies."
            );
            3
        }
        (true, true) => {
            assert!(
                gains[0] == 0.0f64,
                "A Type IV filter must have zero gain at zero frequency."
            );
            4
        }
    };

    let x = linspace(0.0, nyq, nfreqs, true);
    let fx = x.interp(&freqs, gains);

    let mut shift = x
        .iter()
        .map(|&x| {
            Complex::<f64>::new(
                0.0,
                ((numtaps as f64 - 1.0) / -2.0) * std::f64::consts::PI * x / nyq,
            )
        })
        .collect::<Vec<_>>()
        .exp();

    if antisymmetric {
        shift.multiply_const_inplace(Complex::<f64>::new(0.0, 1.0));
    }

    let fx2 = fx
        .iter()
        .map(|x| Complex::<f64>::new(*x, 0.0))
        .collect::<Vec<_>>()
        .multiply(shift.as_slice());

    let out_full = fx2.irfft();

    let mut out = Vec::with_capacity(numtaps);
    unsafe { out.set_len(numtaps) };

    out.copy_from_slice(&out_full[0..numtaps]);

    let hamming = windows::hamming::<f64>(out.len());
    out.multiply_inplace(hamming.as_slice());

    if ftype == 3 {
        let len = out.len();
        out[len / 2] = 0.0;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arange() {
        let range = arange(0, 10, 3);
        println!("{range:?}");
    }

    #[test]
    fn test_linspace() {
        let space = linspace(3.0, 10.0, 3, false);
        println!("{space:?}");
    }

    #[test]
    fn test_firwin2() {
        let space = firwin2(10, &[0.0, 0.5, 0.5, 1.0], &[1.0, 1.0, 0.0, 0.0], false);
        println!("{space:?}");
    }
}
