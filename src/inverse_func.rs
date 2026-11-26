//! [`InverseFunc`]

use frunk::Func;

use crate::{new_new_type_func, type_fn::{BijectiveFunc, BijectiveTypeFunc, TypeFunc}};

/// inverse the function if it is bijective
#[derive(Debug,Default,Clone, Copy)]
pub struct InverseFunc<T>(pub T);
impl<T,I,O> TypeFunc<O> for InverseFunc<T>
	where T:BijectiveTypeFunc<O,Input = I>
{
	type Output=I;
}

impl<T,I,O> BijectiveTypeFunc<I> for InverseFunc<T>
	where T:TypeFunc<I,Output = O>+BijectiveTypeFunc<O,Input = I>
{
	type Input=O;
}

impl<T,I,O> Func<O> for InverseFunc<T> 
	where T:BijectiveFunc<O,Input = I>
{
	type Output=I;

	fn call(i: O) -> Self::Output {
		T::inv_call(i)
	}
}

impl<T,I,O> BijectiveFunc<I> for InverseFunc<T>
	where T:Func<I,Output = O>+BijectiveFunc<O,Input = I>
{
	type Input=O;

	fn inv_call(output:I)->Self::Input {
		T::call(output)
	}
}

new_new_type_func!(InverseFunc pub MapInverse);