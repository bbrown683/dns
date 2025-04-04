// Calls the `From` trait `repeat` times of type `R` using `value` of type `T` and returns a vector of the result
pub trait RepeatFrom<R: Sized,T> where Self: From<T> {
    fn repeat_from(repeat : R, value : T) -> Vec<Self>;
}