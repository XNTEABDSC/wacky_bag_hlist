use std::{marker::PhantomData, ops::Add};

use frunk::{Coproduct, HCons, HNil, Poly, coproduct::CNil, hlist::HMappable};

use crate::new_struct_func;

pub trait CoprodAppend<B> {
    type Output;
}

impl<B> CoprodAppend<B> for CNil {
    type Output=B;
}

impl<H,T,B> CoprodAppend<B> for Coproduct<H,T>
where T:CoprodAppend<B>
{
    type Output = Coproduct<H,T::Output>;
}

pub trait HAllSubsetC {
    type Output;
}

impl HAllSubsetC for HNil {
    type Output=CNil;
}

// impl<H,T> HAllSubsetC for HCons<H,T>
// where T:HAllSubsetC,
//     T::Output:CoprodAppend<

//     >
// {
//     type Output = Coproduct<>;
// }

pub trait HAllSubsetT {
    type Output;
    // fn all_subset(self)->Self::Output;
}

impl HAllSubsetT for HNil {
    type Output=HNil;

    // fn all_subset(self)->Self::Output {
    //     self
    // }
}

new_struct_func!(
    pub FHAppend<A>
    impl<H>:
    ((A,H))->(HCons<A,H>)
);

impl<H,T,TAll,TAllH,R> HAllSubsetT for HCons<H,T>
where T:HAllSubsetT<Output = TAll>+Add<TAllH,Output = R>,
    TAll:HMappable<Poly<FHAppend<H>>,Output = TAllH>
{
    type Output = R;
}
#[cfg(test)]
mod tests{
    use frunk::hlist;

use crate::h_all_subset::HAllSubsetT;

    type A=hlist![String,i32,i64,f32];

    type B=<A as HAllSubsetT>::Output;

    #[test]
    fn test() {
        let b:B=Default::default();
        print!("{:?}",b);
    }

}