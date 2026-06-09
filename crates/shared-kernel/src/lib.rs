pub mod entity;
pub mod error;
pub mod result;
pub mod use_case;
pub mod value_object;
pub mod vo;

pub use entity::{Entity, EntityId};
pub use error::{AppError, DomainError};
pub use result::{combine2, combine_errors, Result};
pub use use_case::UseCase;
pub use value_object::ValueObject;
pub use vo::{Email, Id, Money};

/// Propagate a domain `Result` from nested calls (use instead of `?` on `shared_kernel::Result`).
#[macro_export]
macro_rules! try_domain {
    ($expr:expr) => {
        match $expr {
            $crate::Result::Ok(value) => value,
            $crate::Result::Err(errors) => return $crate::Result::Err(errors),
        }
    };
}
