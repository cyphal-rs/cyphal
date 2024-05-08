/// Represents a Transfer ID
pub trait TransferId<T>: Sized + Default {
    /// Returns the value of the Transfer ID
    fn value(&self) -> T;

    /// Returns the next Transfer ID
    fn next(&self) -> Self;
}
