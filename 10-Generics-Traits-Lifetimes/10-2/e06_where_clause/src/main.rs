use std::fmt::{Display, Debug};

fn main() {
    // We can define this more clearly with a where clause:
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32

    fn some_function<T, U>(t: T, u: U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        0
    }
}
