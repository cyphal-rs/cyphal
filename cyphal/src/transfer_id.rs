pub trait TransferId<T>: Sized {
    fn value(&self) -> T;

    fn next(&self) -> Self;
}
