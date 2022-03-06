use std::ops::Deref;

pub struct Service<S> (pub(crate) S);

impl<S> Deref for Service<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}