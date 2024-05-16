use crate::prelude::*;

pub fn cos_sum<T: DspFloat>(n: usize, alpha: T) -> Vec<T> {
    let mut window = Vec::with_capacity(n);
    unsafe { window.set_len(n) };
    for i in 0..n {
        window[i] = alpha
            - (T::one() - alpha)
                * T::from_f64(2.0 * std::f64::consts::PI * i as f64 / (n - 1) as f64)
                    .expect("Could not convert f64 into type")
                    .cos();
    }
    window
}

pub fn hann<T: DspFloat>(n: usize) -> Vec<T> {
    cos_sum(
        n,
        T::from_f64(0.5).expect("Could not convert f64 into type"),
    )
}

pub fn hamming<T: DspFloat>(n: usize) -> Vec<T> {
    cos_sum(
        n,
        T::from_f64(25.0f64 / 46.0).expect("Could not convert f64 into type"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming() {
        let test = hamming::<f64>(20);
        println!("{test:?}");
    }
}
