use std::fmt::{self, Display, Formatter};

use super::*;

impl<const N: usize, T: Scalar, S: VecAlignment> Display for Vector<N, T, S>
where
    ScalarCount<N>: VecLen<N>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({})",
            self.into_array().map(|c| c.to_string()).join(", ")
        )
    }
}
