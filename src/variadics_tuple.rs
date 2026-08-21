//! Convert bewteen [`HList`] and tuple whose usage implemented by [`variadics_please`]
//! 
//! [`IntoVariadicsTuple`]
//! 
//! [`VariadicsTupleIntoHlist`]

use frunk::{Func, HList, HNil, hlist};
use variadics_please::{all_tuples, all_tuples_enumerated};

use crate::type_fn::{BijectiveFunc, BijectiveTypeFunc, TypeFunc};

/// Convert [`HList`] into tuple form that fits [`all_tuples`] that accepts `impl Trait` and tuple of `impl Trait` up to 16. Nest tuples when more than 16.
/// `HNil` -> ()
/// `HList<T0>` -> `(T0,)` due to implementation limit of `VariadicsTupleIntoHlist`
/// `HList<T0,T1 ...>` -> `(T0,T1,...)`
/// `HList<T0,T1,T...,T14,Then...>` -> `(T0,T1,T...,T14,(Then...))`
pub trait ToVariadicsTuple{
	/// the tuple output
	type Output;
	/// Convert [`HList`] into tuple form that fits [`all_tuples`] that accepts `impl Trait` and tuple of `impl Trait` up to 16. Nest tuples when more than 16
	/// `HNil` -> ()
	/// `HList<T0>` -> `T0`
	/// `HList<T0,T1 ...>` -> `(T0,T1,...)`
	/// `HList<T0,T1,T...,T14,Then...>` -> `(T0,T1,T...,T14,(Then...))`
	fn to_variadics_tuple(self)->Self::Output;
}

impl ToVariadicsTuple for HNil {
	type Output=();

	fn to_variadics_tuple(self)->Self::Output {
		()
	}
}

// impl<T> IntoTupleForm for HCons<T,HNil> {
// 	type Output=T;

// 	fn into_variadics_tuple(self)->Self::Output {
// 		self.head
// 	}
// }


macro_rules! impl_into_variadics_tuple_mid {
	($( ($gts:ident, $nts:ident) ),*) => {
		impl<$($gts),*> ToVariadicsTuple for HList!($($gts,)*){
			#[allow(unused_parens)]
			type Output=($($gts,)*);
			#[allow(unused_variables)]
			fn to_variadics_tuple(self)->Self::Output{
				let s=self;
				$(let ($nts,s) = (s.head,s.tail);)*
				($($nts,)*)
			}
		}
	};
}

all_tuples!(impl_into_variadics_tuple_mid,1,14,T,v);

impl<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,N>
	ToVariadicsTuple for
	HList!(T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,...N) 
	// HCons<T0,HCons<T1,HCons<T2,HCons<T3,HCons<T4,HCons<T5,HCons<T6,HCons<T7,HCons<T8,HCons<T9,HCons<T10,HCons<T11,HCons<T12,HCons<T13,HCons<T14,N>>>>>>>>>>>>>>>
	where N:ToVariadicsTuple
{
	type Output=(T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,N::Output);

	fn to_variadics_tuple(self)->Self::Output {
		let n=self;
		let (v0,n)=(n.head,n.tail);
		let (v1,n)=(n.head,n.tail);
		let (v2,n)=(n.head,n.tail);
		let (v3,n)=(n.head,n.tail);
		let (v4,n)=(n.head,n.tail);
		let (v5,n)=(n.head,n.tail);
		let (v6,n)=(n.head,n.tail);
		let (v7,n)=(n.head,n.tail);
		let (v8,n)=(n.head,n.tail);
		let (v9,n)=(n.head,n.tail);
		let (v10,n)=(n.head,n.tail);
		let (v11,n)=(n.head,n.tail);
		let (v12,n)=(n.head,n.tail);
		let (v13,n)=(n.head,n.tail);
		let (v14,n)=(n.head,n.tail);
		(v0,v1,v2,v3,v4,v5,v6,v7,v8,v9,v10,v11,v12,v13,v14,n.to_variadics_tuple())
	}
}

#[cfg(test)]
mod test{
    use frunk::hlist;

	use super::*;

	#[test]
	fn test() {
		let _a=(hlist![]).to_variadics_tuple();
		let _b=(hlist![1]).to_variadics_tuple();
		let _c=(hlist![1,'2',"3"]).to_variadics_tuple();
		let _d: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, (i32, ))=(
			hlist![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]
		).to_variadics_tuple();
		let _e: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, (i32, i32))=(
			hlist![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17]
		).to_variadics_tuple();
		let _f0=hlist![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33];
		let _f: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, (i32, i32, i32)))=(
			_f0.clone()
		).to_variadics_tuple();
		let _f2=HListIntoVariadicsTuple::inv_call(_f);
		assert_eq!(_f0,_f2);
	}
}
/// convert back from [`IntoVariadicsTuple`]
pub trait VariadicsTupleToHlist {
	/// Output type of [`VariadicsTupleIntoHlist`]
	type Output;
	/// convert back from [`IntoVariadicsTuple`]
	fn to_hlist(self)->Self::Output;
}

impl VariadicsTupleToHlist for () {
	type Output=HNil;

	fn to_hlist(self)->Self::Output {
		HNil
	}
}

macro_rules! impl_variadics_tuple_into_hlist_mid {
	($( ( $enum:tt,  $gts:ident) ),*) => {
		impl<$($gts),*> VariadicsTupleToHlist for ( $($gts,)* ){
			type Output = HList!($($gts),*);
			fn to_hlist(self)->Self::Output{
				hlist![ $(self.$enum),* ]
			}
		}
	};
}

all_tuples_enumerated!(impl_variadics_tuple_into_hlist_mid,1,14,T);


impl<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,N,NO>
	VariadicsTupleToHlist for 
	(T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,N)
	// HCons<T0,HCons<T1,HCons<T2,HCons<T3,HCons<T4,HCons<T5,HCons<T6,HCons<T7,HCons<T8,HCons<T9,HCons<T10,HCons<T11,HCons<T12,HCons<T13,HCons<T14,N>>>>>>>>>>>>>>>
	where N:VariadicsTupleToHlist<Output = NO>
{
	type Output=HList!(T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,... NO) ;

	fn to_hlist(self)->Self::Output {
		// let n=self;
		// let (v0,n)=(n.head,n.tail);
		// let (v1,n)=(n.head,n.tail);
		// let (v2,n)=(n.head,n.tail);
		// let (v3,n)=(n.head,n.tail);
		// let (v4,n)=(n.head,n.tail);
		// let (v5,n)=(n.head,n.tail);
		// let (v6,n)=(n.head,n.tail);
		// let (v7,n)=(n.head,n.tail);
		// let (v8,n)=(n.head,n.tail);
		// let (v9,n)=(n.head,n.tail);
		// let (v10,n)=(n.head,n.tail);
		// let (v11,n)=(n.head,n.tail);
		// let (v12,n)=(n.head,n.tail);
		// let (v13,n)=(n.head,n.tail);
		// let (v14,n)=(n.head,n.tail);
		// (v0,v1,v2,v3,v4,v5,v6,v7,v8,v9,v10,v11,v12,v13,v14,n.into_variadics_tuple())
		hlist![self.0,self.1,self.2,self.3,self.4,self.5,self.6,self.7,self.8,self.9,self.10,self.11,self.12,self.13,self.14,...(self.15.to_hlist())]
	}
}
/// [`IntoVariadicsTuple`] <-> [`VariadicsTupleIntoHlist`]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct HListIntoVariadicsTuple;

impl<A,B> Func<A> for HListIntoVariadicsTuple
	where A:ToVariadicsTuple<Output = B>
{
	type Output=B;

	fn call(i: A) -> Self::Output {
		i.to_variadics_tuple()
	}
}

impl<B,A> BijectiveFunc<B> for HListIntoVariadicsTuple
	where B:VariadicsTupleToHlist<Output = A>,
	A:ToVariadicsTuple<Output = B>
{
	type Input=A;

	fn inv_call(output:B)->Self::Input {
		output.to_hlist()
	}
}


impl<A,B> TypeFunc<A> for HListIntoVariadicsTuple
	where A:ToVariadicsTuple<Output = B>
{
	type Output=B;

}

impl<B,A> BijectiveTypeFunc<B> for HListIntoVariadicsTuple
	where B:VariadicsTupleToHlist<Output = A>,
	A:ToVariadicsTuple<Output = B>
{
	type Input=A;

}