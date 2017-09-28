#[derive(Debug)]
pub enum Error {
    InnerLockError,
    CVWaitError,
    FutureWithCallbackError,
    FutureGoneError,
    ValueError,
}
