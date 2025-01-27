// TODO: generate it automatically with derive-proc-macro, call inject from build-script.

use core::ops::{Try, FromResidual};
use core::convert::Infallible;
use core::ops::ControlFlow;
use crate::ffi::ErrorStatus;


pub type Status = ErrorStatus;


#[derive(Debug, Clone)]
pub struct Error;


impl core::error::Error for Error {}
impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { todo!() }
}


impl From<i8> for ErrorStatus {
	fn from(v: i8) -> Self {
		match v {
			0 => Self::SUCCESS,
			1 => Self::ERROR,
			_ => Self::ERROR,
		}
	}
}

impl From<i8> for Error {
	fn from(_: i8) -> Self { Self }
}

impl From<Error> for ErrorStatus {
	fn from(err: Error) -> Self { unsafe { Self::ERROR } }
}

impl Try for ErrorStatus {
	type Output = ();
	type Residual = Result<Infallible, Error>;

	fn from_output(output: Self::Output) -> Self { ErrorStatus::SUCCESS }

	fn branch(self) -> core::ops::ControlFlow<Self::Residual, Self::Output> {
		match self {
			ErrorStatus::SUCCESS => ControlFlow::Continue(()),
			ErrorStatus::ERROR => ControlFlow::Break(Err(Error)),
		}
	}
}

impl FromResidual for ErrorStatus {
	fn from_residual(residual: <Self as Try>::Residual) -> Self {
		match residual {
			Ok(_) => Self::SUCCESS,
			Err(code) => Self::ERROR,
		}
	}
}
