use num::integer::Roots;

pub trait RadioUnitSqrt<T> {
    fn sqrt(&self) -> T;
}

macro_rules! implement_sqrt_roots {
    ( $type_impl:ident ) => {
        impl RadioUnitSqrt<$type_impl> for $type_impl {
            fn sqrt(&self) -> $type_impl {
                Roots::sqrt(self)
            }
        }
    };
}

macro_rules! implement_sqrt_own {
    ( $type_impl:ident ) => {
        impl RadioUnitSqrt<$type_impl> for $type_impl {
            fn sqrt(&self) -> $type_impl {
                $type_impl::sqrt(*self)
            }
        }
    };
}

implement_sqrt_roots!(u8);
implement_sqrt_roots!(u16);
implement_sqrt_roots!(u32);
implement_sqrt_roots!(u64);
implement_sqrt_roots!(u128);
implement_sqrt_roots!(usize);

implement_sqrt_roots!(i8);
implement_sqrt_roots!(i16);
implement_sqrt_roots!(i32);
implement_sqrt_roots!(i64);
implement_sqrt_roots!(i128);
implement_sqrt_roots!(isize);

implement_sqrt_own!(f32);
implement_sqrt_own!(f64);
