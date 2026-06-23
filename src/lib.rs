#![warn(missing_docs)]
//! Some tool for [`frunk::hlist`]

// pub mod h_type_mappable;

pub mod h_list_helpers;
pub mod h_h_zippable;
pub mod h_extend_by_fn;
pub mod phantom_data_type_params;
pub mod type_fn;
pub mod utils;
pub mod chain_fn;

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

use super::*;
	macro_rules! to_phantom_data_generic_param{
		($($lt:lifetime,)* $($ty:ty),+)=>{( 
			$(& $lt () ,)*
			$($ty),*
		)};
		($($lt:lifetime),* )=>{ ( $(& $lt () ),* ) };
	}

	fn test(){
		// let a:PhantomData<to_phantom_data_generic_param!('static, i32)>=Default::default();
	}
}

// pub struct  awdawd<T,'a>{
	
// }