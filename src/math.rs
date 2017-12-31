use core::intrinsics;
pub use core::f64::consts::PI as PI;

pub trait Float {
	fn cos(self) -> Self;
	fn tan(self) -> Self;
	fn sin(self) -> Self;
	fn floor(self) -> Self;
	fn sqrt(self) -> Self;
	fn exp(self) -> Self;
	fn atan(self) -> Self;
	fn powi(self, n: i32) -> Self;
	fn powf(self, n: Self) -> Self;
	fn to_radians(self) -> Self;
	fn to_degrees(self) -> Self;
	fn ln(self) -> Self;
	fn log_wrapper<F: Fn(Self) -> Self>(self, log_fn: F) -> Self where Self: ::core::marker::Sized;
	fn is_finite(self) -> bool;
	fn is_nan(self) -> bool;
	fn abs(self) -> Self;
}

impl Float for f64 {
	fn abs(self) -> f64 {
		::core::num::Float::abs(self)
	}
	fn is_finite(self) -> bool {
		::core::num::Float::is_finite(self)
	}
	fn is_nan(self) -> bool {
		::core::num::Float::is_nan(self)
	}
	fn ln(self) -> Self {
		self.log_wrapper(|n| { unsafe { intrinsics::logf64(n) } })
	}
	fn cos(self) -> Self {
        unsafe { intrinsics::cosf64(self) }
	}
	fn tan(self) -> Self {
		// unsafe { cmath::tan(self) }
		0.0
	}
	fn sin(self) -> Self {
        unsafe { intrinsics::sinf64(self) }
	}
	fn floor(self) -> Self {
        unsafe { intrinsics::floorf64(self) }
	}
	fn sqrt(self) -> Self {
        if self < 0.0 {
            ::core::f64::NAN
        } else {
            unsafe { intrinsics::sqrtf64(self) }
        }
	}
	fn exp(self) -> Self {
        unsafe { intrinsics::expf64(self) }
	}
	fn atan(self) -> Self {
        // unsafe { cmath::atan(self) }
        0.0
	}
	fn powi(self, n: i32) -> Self {
		::core::num::Float::powi(self, n)
	}
	fn to_radians(self) -> Self {
		::core::num::Float::to_radians(self)
	}
	fn to_degrees(self) -> Self {
		::core::num::Float::to_degrees(self)
	}
	fn powf(self, n: Self) -> Self {
		unsafe { intrinsics::powf64(self, n) }
	}

	// Solaris/Illumos requires a wrapper around log, log2, and log10 functions
	// because of their non-standard behavior (e.g. log(-n) returns -Inf instead
	// of expected NaN).
	fn log_wrapper<F: Fn(f64) -> f64>(self, log_fn: F) -> f64 {
	    if !cfg!(target_os = "solaris") {
	        log_fn(self)
	    } else {
	        if self.is_finite() {
	            if self > 0.0 {
	                log_fn(self)
	            } else if self == 0.0 {
	                ::core::f64::NEG_INFINITY // log(0) = -Inf
	            } else {
	                ::core::f64::NAN // log(-n) = NaN
	            }
	        } else if self.is_nan() {
	            self // log(NaN) = NaN
	        } else if self > 0.0 {
	            self // log(Inf) = Inf
	        } else {
	            ::core::f64::NAN // log(-Inf) = NaN
	        }
	    }
	}
}
