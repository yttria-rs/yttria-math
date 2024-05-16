use num::{complex::ComplexFloat, Complex, Float};
use rayon::prelude::*;
use rustfft::{FftNum, FftPlanner};

use super::GenericVectorMath;

pub trait DspComplex: ComplexFloat + Float + Send + Sync + Copy {}
impl<T> DspComplex for T where T: ComplexFloat + Float + Send + Sync + Copy {}

pub trait ComplexVectorMath<T> {
    fn real(&self) -> Vec<T>;
    fn imag(&self) -> Vec<T>;

    fn conj(&self) -> Vec<Complex<T>>;
    fn conj_inplace(&mut self);

    fn exp_into(&self, out: &mut [Complex<T>]);
    fn exp(&self) -> Vec<Complex<T>>;
    fn exp_inplace(&mut self);
}

impl<T> ComplexVectorMath<T> for [Complex<T>]
where
    T: DspComplex,
{
    fn real(&self) -> Vec<T> {
        self.iter().map(|x| x.re).collect()
    }

    fn imag(&self) -> Vec<T> {
        self.iter().map(|x| x.im).collect()
    }

    fn conj(&self) -> Vec<Complex<T>> {
        self.par_iter().map(|x| x.conj()).collect()
    }

    fn conj_inplace(&mut self) {
        self.par_iter_mut().for_each(|x| {
            *x = x.conj();
        })
    }

    fn exp_into(&self, out: &mut [Complex<T>]) {
        out.par_iter_mut()
            .zip(self)
            .for_each(|(out, own)| *out = own.exp());
    }

    fn exp(&self) -> Vec<Complex<T>> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) }
        self.exp_into(out.as_mut_slice());
        out
    }

    fn exp_inplace(&mut self) {
        todo!()
    }
}

pub trait FftVectorMath<T> {
    fn fft_into(&self, out: &mut [Complex<T>], scratch: &mut [Complex<T>]);
    fn fft(&self) -> Vec<Complex<T>>;

    fn ifft_into(&self, out: &mut [Complex<T>], scratch: &mut [Complex<T>]);
    fn ifft(&self) -> Vec<Complex<T>>;

    fn irfft_into(&self, out: &mut [T], scratch: &mut [Complex<T>]);
    fn irfft(&self) -> Vec<T>;
}

impl<T> FftVectorMath<T> for [Complex<T>]
where
    T: FftNum + DspComplex,
{
    fn fft_into(&self, out: &mut [Complex<T>], scratch: &mut [Complex<T>]) {
        let mut planner = FftPlanner::<T>::new();
        let fft = planner.plan_fft_forward(self.len());

        out[0..(self.len())].clone_from_slice(self);

        fft.process_with_scratch(out, scratch);
        out.divide_const_inplace(Complex::<T>::new(
            T::from_usize(self.len()).expect("Could not convert array size to type"),
            T::zero(),
        ));
    }

    fn fft(&self) -> Vec<Complex<T>> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) }
        let mut scratch = Vec::with_capacity(self.len());
        unsafe { scratch.set_len(self.len()) }

        self.fft_into(out.as_mut_slice(), scratch.as_mut_slice());
        out
    }

    fn ifft_into(&self, out: &mut [Complex<T>], scratch: &mut [Complex<T>]) {
        out[0..(self.len())].clone_from_slice(self);

        let mut planner = FftPlanner::<T>::new();
        let ifft = planner.plan_fft_inverse(self.len());

        ifft.process_with_scratch(out, scratch);
        out.divide_const_inplace(Complex::<T>::new(
            T::from_usize(self.len()).expect("Could not convert array size to type"),
            T::zero(),
        ));
    }

    fn ifft(&self) -> Vec<Complex<T>> {
        let mut out = Vec::with_capacity(self.len());
        unsafe { out.set_len(self.len()) }
        let mut scratch = Vec::with_capacity(self.len());
        unsafe { scratch.set_len(self.len()) }

        self.ifft_into(out.as_mut_slice(), scratch.as_mut_slice());
        out
    }

    fn irfft_into(&self, out: &mut [T], scratch: &mut [Complex<T>]) {
        let out_len = 2 * (self.len() - 1);
        let mut hermitian = Vec::with_capacity(2 * self.len() - 1);
        unsafe { hermitian.set_len(2 * self.len() - 1) }

        hermitian[0..(self.len())].clone_from_slice(&self[0..(self.len())]);
        hermitian.conj_inplace();
        hermitian.reverse();
        hermitian[0..(self.len())].clone_from_slice(&self[0..(self.len())]);

        hermitian.resize(
            out_len * 2,
            Complex {
                re: T::zero(),
                im: T::zero(),
            },
        );

        let mut planner = FftPlanner::<T>::new();
        let ifft = planner.plan_fft_inverse(out.len());

        ifft.process_with_scratch(hermitian.as_mut_slice(), scratch);
        hermitian.divide_const_inplace(Complex::<T>::new(
            T::from_usize(out_len).expect("Could not convert array size to type"),
            T::zero(),
        ));

        out.clone_from_slice(&hermitian[0..(out.len())].real());
    }

    fn irfft(&self) -> Vec<T> {
        let out_len = 2 * (self.len() - 1);
        let mut out = Vec::with_capacity(out_len);
        unsafe { out.set_len(out_len) }
        let mut scratch = Vec::with_capacity(out_len);
        unsafe { scratch.set_len(out_len) }

        self.irfft_into(out.as_mut_slice(), scratch.as_mut_slice());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::complex::Complex32;

    #[test]
    fn test_real() {
        let test = vec![
            Complex32 { re: 0.0, im: 0.0 },
            Complex32 { re: 1.0, im: 2.0 },
            Complex32 { re: 2.0, im: 5.0 },
            Complex32 { re: 3.0, im: 7.0 },
            Complex32 { re: 4.0, im: 9.0 },
        ];

        let _split = test.real();
    }

    #[test]
    fn test_ifft() {
        let test = vec![
            Complex32 { re: 1.0, im: 0.0 },
            Complex32 { re: 0.0, im: -1.0 },
            Complex32 { re: -1.0, im: 0.0 },
        ];

        let fft = test.ifft();
        println!("{fft:?}");
    }

    #[test]
    fn test_irfft() {
        let test = vec![
            Complex32 { re: 1.0, im: 0.0 },
            Complex32 { re: 0.0, im: -1.0 },
            Complex32 { re: -1.0, im: 0.0 },
            Complex32 { re: 2.0, im: 0.0 },
            Complex32 { re: 0.0, im: 3.0 },
        ];

        let fft = test.irfft();
        println!("{fft:?}");
    }
}
