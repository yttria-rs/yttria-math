use super::generic::DspGeneric;
use num::Integer;

pub trait DspInt: DspGeneric + Integer {}
impl<T> DspInt for T where T: DspGeneric + Integer {}

pub trait IntegerVectorMath<T> {}

impl<T: DspInt> IntegerVectorMath<T> for [T] {}
