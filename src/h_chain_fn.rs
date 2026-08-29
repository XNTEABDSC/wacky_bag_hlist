//! Chain [`HList`] [`Func`]s as 1
//! [`ChainFunc`]

use std::marker::PhantomData;

use frunk::{Func, HNil, Poly, hlist::{HFoldLeftable, HFoldRightable, HMappable}};

use crate::{h_list_helpers::FoldApply, impl_func, impl_phantom, new_struct_func, reverse_func::MapReverse, type_fn::{BijectiveFunc, BijectiveTypeFunc, TypeFunc}};

/// chain n functions together 
/// 
/// call left to right
#[derive(Debug,Default,Clone, Copy)]
pub struct HChainFunc<F>(pub F);

impl<I,O,F> Func<I> for HChainFunc<F>
where 
	F:Default + HFoldLeftable<Poly<FoldApply>,I,Output = O>
{
	type Output=O;

	fn call(i: I) -> Self::Output {
		F::default().foldl(Poly(FoldApply), i)
	}
}

impl<I,O,F> BijectiveFunc<O> for HChainFunc<F>
where 
	F:Default + HMappable<Poly< MapReverse >, Output : HFoldRightable<Poly<FoldApply>,O,Output = I> > + HFoldLeftable<Poly<FoldApply>,I,Output = O>
{
	type Input=I;

	fn inv_call(output:O)->Self::Input {
		F::default().map(Poly(MapReverse)).foldr(Poly(FoldApply), output)
	}
}
#[cfg(test)]
mod test{
	
    use frunk::{Func, Poly, hlist};

use crate::{h_chain_fn::HChainFunc, h_list_helpers::FoldApply, new_new_type_func, reverse_func::ReverseFunc};

	struct AWD<T>(pub T);
	new_new_type_func!(AWD MapAWD);

	struct BWD<T>(pub T);
	new_new_type_func!(BWD MapBWD);
	#[test]
	fn test(){
		let fawd=hlist![MapAWD,MapBWD];
		let f=HChainFunc(fawd);
		let v=1;
		// let fv=fawd.foldl(Poly(FoldApply), v);
		let fv=FoldApply::call((v,f));
		let v=FoldApply::call( (fv,ReverseFunc(f))  );
		assert_eq!(v,1);
	}
}