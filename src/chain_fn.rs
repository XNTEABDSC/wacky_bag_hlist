use frunk::Func;

use crate::type_fn::{BijectiveFunc, BijectiveTypeFunc, TypeFunc};


/// chain 2 functions together 
/// 
/// call 0 then 1
#[derive(Default,Debug,Clone, Copy)]
pub struct ChainFunc<F1,F2>(pub F1,pub F2);

impl<F1,F2,V1,V2,V3> TypeFunc<V1> for ChainFunc<F1,F2>
	where F1:TypeFunc<V1,Output = V2>,
	F2:TypeFunc<V2,Output = V3>
{
	type Output=V3;
}

impl<F1,F2,V1,V2,V3> BijectiveTypeFunc<V3> for ChainFunc<F1,F2>
	where F2:BijectiveTypeFunc<V3,Input = V2>,
	F1:BijectiveTypeFunc<V2,Input = V1>
{
	type Input = V1;
}

impl<F1,F2,V1,V2,V3> Func<V1> for ChainFunc<F1,F2> 
	where F1:Func<V1,Output = V2>,
	F2:Func<V2,Output = V3>
{
	type Output=V3;

	fn call(i: V1) -> Self::Output {
		F2::call(F1::call(i))
	}
}

impl<F1,F2,V1,V2,V3> BijectiveFunc<V3> for ChainFunc<F1,F2> 
	where F2:BijectiveFunc<V3,Input = V2>,
		F1:BijectiveFunc<V2,Input = V1>
{
	type Input = V1;
	
	fn inv_call(output:V3)->Self::Input {
		F1::inv_call(F2::inv_call(output))
	}
}