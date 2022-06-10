
/// Generate the [From<T>] trait implementation for an custom error struct with an specific
/// structure, with ErrorKind, and message attributes;
///
///
/// # Example
/// ```
/// use std::fmt::{Display, Formatter, Result, Debug};
/// use std::io::Error as ioError;
/// use std::env::VarError;
/// use heimdall::errors::ErrorKind::{
///     Io,
///     Env,
/// };
/// use heimdall::implement_error;
///
/// // First, you need create your Error struct
/// #[derive(Debug, PartialEq, Clone)]
/// pub (crate) struct MyErrorType {
///     kind: heimdall::errors::ErrorKind,
///     message: String
/// }
///
/// // Implement the Display trait
/// impl Display for MyErrorType {
///     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
///         write!(
///             f,
///             "kind: {} message: {}",
///             self.kind.to_string(),
///             self.message
///         )
///     }
/// }
///
/// // Generate implementations
/// implement_error!(MyErrorType, ioError, Io);
/// implement_error!(MyErrorType, VarError, Env);
///
/// ```
///
#[macro_export]
macro_rules! implement_error {
    ($err:ident, $t: ident, $kind: ident) => {
        impl From<$t> for $err {
            fn from(error: $t) -> $err {
                $err {
                    kind: $kind,
                    message: error.to_string(),
                }
            }
        }
    }
}