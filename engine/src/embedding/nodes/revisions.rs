use moxie::runtime::Revision;
pub trait Previous {
    fn previous() -> Self;
}

impl Previous for Revision {
    fn previous() -> Self {
        Revision(Revision::current().0 - 1)
    }
}