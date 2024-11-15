use std::fmt;
use std::fmt::Formatter;

pub(crate) struct NoDebug<T>(pub T);

impl<T> fmt::Debug for NoDebug<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "∅")
    }
}
