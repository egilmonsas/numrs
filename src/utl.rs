use crate::{EPSILON, F};

pub trait ZEq<T> {
    fn zeq(&self, other: T) -> bool;

    fn zneg(&self, other: T) -> bool {
        !self.zeq(other)
    }
}

impl ZEq<F> for F {
    fn zeq(&self, other: F) -> bool {
        return (*self - other).abs() < F::from(EPSILON * 2.0);
    }
}
//Might need to be revisited
#[macro_export]
macro_rules! assert_zeq {
    ($left:expr,$right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left, right) => {
                if left.zneg(*right) {
                    panic!(
                        "asserting zequality between {:?} and {:?} failed",
                        left, right
                    )
                }
            }
        }
    }};
}
#[macro_export]
macro_rules! assert_nzeq {
    ($left:expr,$right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left, right) => {
                if left.zeq(*right) {
                    panic!(
                        "asserting inzequality between {:?} and {:?} failed",
                        left, right
                    )
                }
            }
        }
    }};
}
