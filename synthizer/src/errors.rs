use synthizer_sys::*;

pub type Result<T> = std::result::Result<T, Error>;

/// An ErrorKind represents what kind of error Synthizer has given back.
/// Currently, this is only `Other`, since Synthizer hasn't yet defined error
/// codes properly.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum ErrorKind {
    Other,
}

#[derive(Clone, Debug)]
pub struct Error {
    message: String,
    kind: ErrorKind,
    code: syz_ErrorCode,
}

impl Error {
    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub fn get_kind(&self) -> ErrorKind {
        self.kind
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "Synthizer error {}: {}",
            self.code,
            self.message.as_str()
        )?;
        Ok(())
    }
}

impl std::error::Error for Error {}

/// Generate a Synthizer error from a code, collecting the last message as needed.
pub(crate) fn error_from_code(code: syz_ErrorCode) -> Error {
    let msg_raw = unsafe { syz_getLastErrorMessage() };
    let msg_c = unsafe { std::ffi::CStr::from_ptr(msg_raw).to_string_lossy() };
    Error {
        kind: ErrorKind::Other,
        message: msg_c.into_owned(),
        code,
    }
}

/// Return `Ok` if the error code is a success. Otherwise, convert to an error.
/// Convenience helper to use `?` with Synthizer.
pub(crate) fn check_error(code: syz_ErrorCode) -> Result<()> {
    if code != 0 {
        return Err(error_from_code(code));
    }

    Ok(())
}
