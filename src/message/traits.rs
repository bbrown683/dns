use bytes::BytesMut;

// Calls the `From` trait `repeat` times of type `R` using `value` of type `T` and returns a vector of the result
pub trait RepeatToVec<R: Sized, T> where Self: From<T> {
    fn repeat_to_vec(repeat : R, value : T) -> Vec<Self>;
}

pub trait RepeatToBytes<T> where Self: From<T> {
    fn repeat_to_bytes(value : Vec<T>) -> BytesMut;
}