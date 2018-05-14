use failure;

pub type Result<T> = ::std::result::Result<T, failure::Error>;

// Fix upstream usage of error-chain
// TODO(robertgzr): maybe do a PR switching darwin_rs to failure as well?
//
// copypasta from:
// https://github.com/rust-lang-nursery/failure/issues/109#issuecomment-350920299
//
pub trait ResultExt<T, E> {
    fn sync(self) -> ::std::result::Result<T, failure::SyncFailure<E>>
    where
        Self: Sized,
        E: ::std::error::Error + Send + 'static;
}

impl<T, E> ResultExt<T, E> for ::std::result::Result<T, E> {
    fn sync(self) -> ::std::result::Result<T, failure::SyncFailure<E>>
    where
        Self: Sized,
        E: ::std::error::Error + Send + 'static,
    {
        self.map_err(failure::SyncFailure::new)
    }
}
