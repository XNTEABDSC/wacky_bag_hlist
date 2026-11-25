use std::marker::PhantomData;

use frunk::{Func, HCons, HList, HNil, indices::{Here, There}};

use crate::{chain_fn::ChainFunc, h_list_helpers::FIdent, type_fn::TypeFunc};

pub struct MapThere;

impl<T> TypeFunc<T> for MapThere {
    type Output=There<T>;
}

pub trait HWithIndex<Prepend>{
    type Output;
    fn with_index(self)->Self::Output;
}

impl<Prepend> HWithIndex<Prepend> for HNil {
    type Output=HNil;
    
    fn with_index(self)->Self::Output {
        HNil
    }
}

impl<H,T,Prepend,PrependedHere> HWithIndex<Prepend> for HCons<H,T>
where T:HWithIndex< ChainFunc<MapThere,Prepend> >,
    Prepend:TypeFunc<Here,Output = PrependedHere>
{
    type Output=HCons<
        (H,PhantomData<PrependedHere>),
        T::Output
    >;
    
    fn with_index(self)->Self::Output {
        HCons{
            head:(self.head,Default::default()),
            tail:self.tail.with_index()
        }
    }
}

fn test() {
    type A=HList!(u32,u64);
    type B=<A as HWithIndex<FIdent>>::Output;

    let b=B::default();
}