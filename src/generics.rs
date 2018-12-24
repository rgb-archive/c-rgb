pub trait WrapperOf<T> {
    fn decode(&self) -> T;
    fn encode(native: &T) -> Self;
}