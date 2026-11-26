use std::{marker::PhantomData, ops::Add};

use frunk::{Coproduct, HCons, HList, HNil, Poly, coproduct::{CNil, CoproductMappable}, hlist::HMappable};

use crate::{c_append::CoprodAppend, h_list_helpers::HTypeFnToMapper, new_struct_func};

/// hlist Self to coprod of all subset of Self's element (2^n)
/// 
/// just for fun.
pub trait HAllSubsetC {
    /// coprod of all subset of Self's element (2^n)
    type Output;
}

impl HAllSubsetC for HNil {
    type Output=Coproduct<HNil,CNil>;
}

impl<H,T,TAll,TAllH,R> HAllSubsetC for HCons<H,T>
where 
    T:
        HAllSubsetC<Output = TAll>,
    TAll:
        CoproductMappable< Poly<HTypeFnToMapper<FHAppend<H>>>,
        Output = TAllH>,
    TAllH:CoprodAppend<TAll,Output = R>
{
    type Output = R;
}

/// hlist Self to hlist of all subset of Self's element (2^n)
/// 
/// just for fun.
pub trait HAllSubsetT {
    /// hlist of all subset of Self's element (2^n)
    type Output;
    
}

impl HAllSubsetT for HNil {
    type Output=HList!(HNil);

    // fn all_subset(self)->Self::Output {
    //     self
    // }
}

new_struct_func!(
    pub FHAppend<A>
    impl<H>:
    (H)->(HCons<A,H>)
);

impl<H,T,TAll,TAllH,R> HAllSubsetT for HCons<H,T>
where 
    T:
        HAllSubsetT<Output = TAll>,
    TAll:
        HMappable< Poly<HTypeFnToMapper<FHAppend<H>>>,
        Output = TAllH>,
    TAllH:Add<TAll,Output = R>
{
    type Output = R;
}

// you dont really want this right

#[cfg(test)]
mod tests{
    use std::marker::PhantomData;

    use frunk::{Func, HList, HNil, Poly, hlist::HMappable};

    use crate::{chain_fn::ChainFunc, h_all_subset::{FHAppend, HAllSubsetC, HAllSubsetT}, h_chain_fn::HChainFunc, h_list_helpers::MapToPhantom, type_fn::{MapFromPhantomDataPanic, TypeFnAsPhantomFn}};

    // type A=HList![String,i32,i64,f32];
    type A=HList![String,i32];

    type B=<A as HAllSubsetT>::Output;

    type C=<A as HAllSubsetC>::Output;

    #[test]
    fn test() {
        let b: B =Default::default();
        print!("{:?}",b);

        let c:C=C::inject(HNil);
        print!("{:?}",c);
    }

}