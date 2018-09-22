pub trait Compose: Sized {
    fn compose<A>(self, other: A)
                           -> (Self, A) {
        (self, other)
    }
}

impl<A> Compose for A {}

macro_rules! compose {
( $first:expr ) => ( ($first) );
( $first:expr, $second:expr, ) => ( ($first, $second) );
( $first:expr, $( $x:expr ),* ) => {
        {
            ($first, compose!( $($x),*))
        }
};
}
