//! Process hlist by fn, sculpt, f, then add
//! 
//! [`h_extend_by_fn`]
//! [`h_extend_by_fn_ref`]

use std::ops::Add;

use frunk::{ToRef, hlist::Sculptor};



/// `f` sculpt params `FI` from `v` and generates `FO`
pub fn h_extend_by_fn<V,Idx,F,FI,FO,O>(v:V,f:F)->O
where F:FnOnce(FI)->FO,
	V:Sculptor<FI,Idx,Remainder : Add<FO,Output = O>>,
{
	let (fi,r)=v.sculpt();
	let fo=f(fi);
	r+fo
}

/// `f` sculpt params `FI` from `v.to_ref()` and generates `FO`
pub fn h_extend_by_fn_ref<V,Idx,F,FI,FO,O>(v:V,f:F)->O
where 
	V:Add<FO,Output = O>,
	for<'a> V:ToRef<'a,Output : Sculptor<FI,Idx>>,
	F:FnOnce(FI)->FO,
{
	let fi=v.to_ref().sculpt().0;
	let fo=f(fi);
	v+fo
}

// /// `f` sculpt params `FI` from `v.to_ref()` and generates `FO`
// pub fn h_extend_by_fn_ref<V,F,Idx,Rem,FI,FIR,FO,O>(v:V,f:F)->O
// where 
// 	F:FnOnce(FIR)->FO,
// 	V:Add<FO,Output = O>,
// 	V:Sculptor<FI,Idx,Remainder = Rem>,
// 	FI:for<'a> HMappable<Poly<MapRef<'a>>,Output = FIR>
// {
// 	let fi=v.to_ref().sculpt().0;
// 	let fo=f(fi);
// 	v+fo
// }

// pub fn h_apply_fn_inplace<>