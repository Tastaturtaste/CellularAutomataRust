type AddOut<T> = <T as std::ops::Add>::Output;
pub fn add_tuples<T>(tup1: &(T, T), tup2: &(T, T)) -> (AddOut<T>, AddOut<T>)
where
    T: std::ops::Add + Copy,
{
    (tup1.0 + tup2.0, tup1.1 + tup2.1)
}
