//! [WithTypeHintBijective]
//! [WithTypeHint]
use frunk::Func;

use crate::type_fn::{BijectiveFunc, BijectiveTypeFunc, TypeFunc};

/// sometimes `F` does not provides enough requirement to 
pub struct WithTypeHintBijective<F,TF>(pub F,pub TF);

impl<F,TF,I,O> TypeFunc<I> for WithTypeHintBijective<F,TF>
where TF:TypeFunc<I,Output = O>,
    F:TypeFunc<I,Output = O>
{
    type Output=O;
}

impl<F,TF,I,O> BijectiveTypeFunc<O> for WithTypeHintBijective<F,TF>
where TF:BijectiveTypeFunc<O,Input = I>,
    F:BijectiveTypeFunc<O,Input = I>
{
    type Input=I;
}

impl<F,TF,I,O> Func<I> for WithTypeHintBijective<F,TF>
where TF:TypeFunc<I,Output = O>,
    F:Func<I,Output = O>
{
    type Output=O;

    fn call(i: I) -> Self::Output {
        F::call(i)
    }
}

impl<F,TF,I,O> BijectiveFunc<O> for WithTypeHintBijective<F,TF>
where TF:BijectiveTypeFunc<O,Input = I>,
    F:BijectiveFunc<O,Input = I>
{
    type Input = I;
    fn inv_call(output:O)->Self::Input {
        F::inv_call(output)
    }
}


/// sometimes `F` does not provides enough requirement to 
pub struct WithTypeHint<F,TF>(pub F,pub TF);

impl<F,TF,I,O> TypeFunc<I> for WithTypeHint<F,TF>
where TF:TypeFunc<I,Output = O>,
    F:TypeFunc<I,Output = O>
{
    type Output=O;
}

impl<F,TF,I,O> BijectiveTypeFunc<O> for WithTypeHint<F,TF>
where TF:BijectiveTypeFunc<O,Input = I>,
    F:TypeFunc<I,Output = O>
{
    type Input=I;
}

impl<F,TF,I,O> Func<I> for WithTypeHint<F,TF>
where TF:TypeFunc<I,Output = O>,
    F:Func<I,Output = O>
{
    type Output=O;

    fn call(i: I) -> Self::Output {
        F::call(i)
    }
}

impl<F,TF,I,O> BijectiveFunc<O> for WithTypeHint<F,TF>
where TF:BijectiveTypeFunc<O,Input = I>,
    F:BijectiveFunc<O,Input = I>
{
    type Input = I;
    fn inv_call(output:O)->Self::Input {
        F::inv_call(output)
    }
}