//! [`HSelectZippable`]

use frunk::{Poly, hlist::{HMappable, HZippable, Sculptor}};

use crate::h_list_helpers::{HTypeFnToMapper, HTypeMapP, HZip};

/// sculpt b, select with TypeFunc, then zip
pub fn h_sculpt_select_zip<A,TypeFunc,B,BIdx>(a:A,_tf:TypeFunc,b:B)
-> (HZip< A, HTypeMapP<A,TypeFunc>>, <B as Sculptor< HTypeMapP<A,TypeFunc>, BIdx>>::Remainder)
where 
	A:HMappable< Poly<HTypeFnToMapper<TypeFunc>> >,
	B:Sculptor< HTypeMapP<A,TypeFunc>, BIdx>,
	A:HZippable< HTypeMapP<A,TypeFunc> >
{
	let (b1,b2)=b.sculpt();
	(a.zip(b1),b2)
}

/// select b with TypeFunc, then zip
/// We dont need a new trait to do this!
pub fn h_select_zip<A,TypeFunc,B>(a:A,_tf:TypeFunc,b:B)
-> HZip< A, HTypeMapP<A,TypeFunc>>
where 
	A:HMappable< Poly<HTypeFnToMapper<TypeFunc>> ,Output = B>,
	A:HZippable< B >
{
	a.zip(b)
}


/// select b with TypeFunc
pub fn h_select_by_a_fn<A,TypeFunc,B>(b:B,_a:&A,_tf:TypeFunc)
-> B
where 
	A:HMappable< Poly<HTypeFnToMapper<TypeFunc>> ,Output = B>,
{
	b
}

// pub trait HSelectZippable<TypeFunc,B,BIdx> {
// 	type Output;
// 	type Remainder;

// }

// impl<A,TypeFunc,B,BIdx,BSel> HSelectZippable<TypeFunc,B,BIdx> for A 
// where 
// 	A:HMappable< Poly<HTypeFnToMapper<TypeFunc>> ,Output = BSel>,
// 	B:Sculptor< BSel, BIdx>,
// 	A:HZippable< BSel >
// {
// 	type Output = HZip<A,BSel>;
// 	type Remainder = <B as Sculptor< HTypeMapP<A,TypeFunc>, BIdx>>::Remainder;
// }

// /// select b with [TypeFunc], and zip
// pub trait HSelectZippable<TypeFunc,B>
// {
// 	/// the result
// 	type Output;
// 	/// select b with tf, and zip
// 	fn select_zip(self,tf:TypeFunc,b:B)->Self::Output;
// }

// impl<TF> HSelectZippable<TF,HNil> for HNil {
// 	type Output=HNil;

// 	fn select_zip(self,_tf:TF,_b:HNil)->Self::Output {
// 		HNil
// 	}
// }

// impl<TF,TInputH,TInputT,BInputH,BInputT> HSelectZippable<Poly<TF>,HCons<BInputH,BInputT>> for HCons<TInputH,TInputT>
// 	where TF:TypeFunc<TInputH,Output = BInputH>,
// 		TInputT:HSelectZippable<Poly<TF>,BInputT>
// {
// 	type Output=HCons<(TInputH,BInputH),TInputT::Output>;

// 	fn select_zip(self,tf:Poly<TF>,b:HCons<BInputH,BInputT>)->Self::Output {
// 		HCons{head:(self.head,b.head),tail:self.tail.select_zip(tf, b.tail)}
// 	}
// }