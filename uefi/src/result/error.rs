use super::Status;
use core::fmt::{Debug, Display};

/// Errors emitted from UEFI entry point must propagate erronerous UEFI statuses,
/// and may optionally propagate additional entry point-specific data.
#[derive(Debug, PartialEq, Eq)]
pub struct Error<Data: Debug = ()> {
    status: Status,
    data: Data,
}

impl<Data: Debug> Error<Data> {
    /// Create an `Error`.
    pub const fn new(status: Status, data: Data) -> Self {
        Self { status, data }
    }

    /// Get error `Status`.
    pub const fn status(&self) -> Status {
        self.status
    }

    /// Get error data.
    pub const fn data(&self) -> &Data {
        &self.data
    }

    /// Split this error into its inner status and error data
    #[allow(clippy::missing_const_for_fn)]
    pub fn split(self) -> (Status, Data) {
        (self.status, self.data)
    }
}

// Errors without error data can be autogenerated from statuses

impl From<Status> for Error<()> {
    fn from(status: Status) -> Self {
        Self { status, data: () }
    }
}

impl<Data: Debug + Display> Display for Error<Data> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "UEFI Error {}: {}", self.status(), self.data())
    }
}

#[cfg(feature = "unstable")]
impl<Data: Debug + Display> core::error::Error for Error<Data> {}
