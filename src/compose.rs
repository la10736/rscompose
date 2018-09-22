pub trait Compose: Sized {
    fn compose<A: Compose>(self, other: A)
                           -> (Self, A) {
        (self, other)
    }
}

impl<A: Compose, B: Compose> Compose for (A, B) {}

macro_rules! compose {
( $first:expr ) => ( ($first) );
( $first:expr, $second:expr, ) => ( ($first, $second) );
( $first:expr, $( $x:expr ),* ) => {
        {
            ($first, compose!( $($x),*))
        }
};
}
