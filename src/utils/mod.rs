//! utils

pub mod impl_phantom;
pub mod phantom_data_type_params;

/// showing that F is `impl FnOnce(I)->O`
pub fn restrict_fn_once_type<F,I,O>(f:F)->F
where F:FnOnce(I)->O
{
	f
}