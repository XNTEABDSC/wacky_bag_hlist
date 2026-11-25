use frunk::{Coproduct, HCons, HNil, coproduct::CNil};

use crate::type_fn::{BijectiveTypeFunc, TypeFunc};


/// convert HList type to Coprod
pub trait HListToCoprod {
    /// the Coprod type
    type Coprod;
}

impl HListToCoprod for HNil {
    type Coprod=CNil;
}

impl<H,T> HListToCoprod for HCons<H,T>
where T:HListToCoprod
{
    type Coprod = Coproduct<H,T::Coprod>;
}

/// convert Coprod type to HList
pub trait CoprodToHList {
    /// the HList type
    type HList;
}

impl CoprodToHList for CNil {
    type HList=HNil;
}

impl<H,T> CoprodToHList for Coproduct<H,T>
where T:CoprodToHList
{
    type HList=HCons<H,T::HList>;
}
/// HList type to Coprod
pub struct TFHListToCoprod;

impl<I,O> TypeFunc<I> for TFHListToCoprod
where I:HListToCoprod<Coprod = O>,
    O:CoprodToHList<HList = I>
{
    type Output=O;
}

impl<I,O> BijectiveTypeFunc<O> for TFHListToCoprod
where I:HListToCoprod<Coprod = O>,
    O:CoprodToHList<HList = I>
{
    type Input=I;
}