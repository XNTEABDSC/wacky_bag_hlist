//! utils

pub mod impl_phantom;

/// showing that F is `impl FnOnce(I)->O`
pub fn restrict_fn_once_type<F,I,O>(f:F)->F
where F:FnOnce(I)->O
{
	f
}