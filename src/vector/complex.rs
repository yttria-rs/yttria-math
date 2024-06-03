use num::{complex::ComplexFloat, Complex, Float};
use rayon::prelude::*;

pub trait YttriaVectorComplex<T> {
    fn real(&self) -> Vec<T>;
    fn imag(&self) -> Vec<T>;

    fn conj(&self) -> Vec<Complex<T>>;
    fn conj_inplace(&mut self);

    fn exp_into(&self, out: &mut [Complex<T>]);
    fn exp(&self) -> Vec<Complex<T>>;
    fn exp_inplace(&mut self);
}

impl<T> YttriaVectorComplex<T> for [Complex<T>]
where
    T: ComplexFloat + Float + Send + Sync + Copy,
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
}
