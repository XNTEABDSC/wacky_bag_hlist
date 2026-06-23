//! some helper things for h_list

use std::{iter::Chain, marker::PhantomData, ops::{Add, Deref, DerefMut, Neg}};

use frunk::{Func, HCons, HList, HNil, Poly, ToMut, ToRef, hlist::{HMappable, HZippable}};

use crate::{chain_fn::ChainFunc, impl_phantom, new_struct_func, type_fn::{BijectiveTypeFunc, MapFromPhantomDataPanic, TypeFnAsPhantomFn, TypeFunc}};

new_struct_func!(
	pub MapToPhantom
	impl<T>: 
	(T) <-> (PhantomData<T>) 
	|_i|Default::default()
);

new_struct_func!(
	pub FoldChainIter impl <Acc,X,Item>
	{where Acc:Iterator<Item = Item>,
		X:Iterator<Item = Item>}
	:
	((Acc,X)) <-> (Chain<Acc,X>)
	|i|i.0.chain(i.1)
);

new_struct_func!{
	pub MapDeref
	impl <'a,TA,TB>
	{where TA:Deref<Target=TB>+'a,TB:'a}:
	(&'a TA) -> (&'a TB)
	|i|i.deref()
}

/// `ta:TA` -> `ta.deref():TB`, with TF: TA <-> TB specified by `TF`
pub struct MapDerefT<TF>(PhantomData<TF>);

impl_phantom!(MapDerefT<T>);

impl<'a,TF,TA,TB> TypeFunc<&'a TA> for MapDerefT<TF> 
	where 
		TA:Deref<Target=TB>,
		TB:'a,
		TF:TypeFunc<TA,Output = TB>
{
	type Output=&'a TB;
}

impl<'a,TF,TA,TB> BijectiveTypeFunc<&'a TB> for MapDerefT<TF> 
	where 
		TA:Deref<Target=TB>,
		TB:'a,TA:'a,
		TF:BijectiveTypeFunc<TB,Input = TA>
{
	type Input =&'a TA;
}

impl<'a,TF,TA,TB> Func<&'a TA> for MapDerefT<TF>
	where 
		TA:Deref<Target=TB>,
		TB:'a,
		TF:TypeFunc<TA,Output = TB>

{
	type Output=&'a TB;

	fn call(i: &'a TA) -> Self::Output {
		i.deref()
	}
}

new_struct_func!{
	pub MapClone
	impl <'a,T> {where T:Clone+'a}:
	(&'a T) -> (T)
	|i|i.clone()
}

new_struct_func!{
	pub MapMutToRef
	impl<'a,T> {where T:'a}:
	(&'a mut T) <-> (&'a T)
	|i|i
}

new_struct_func!{
	pub MapNeg
	impl <T> {where T:Neg}:
	(T) -> (T::Output)
	|i|-i
}

new_struct_func!{
	pub MapNegRev
	impl <T1,T2>
	{where T1:Neg<Output = T2>,
		T2:Neg<Output = T1>}:
	(T1) <-> (T2)
	|i|-i,
	|i|-i
}

new_struct_func!{
	pub MapRef<'a>
	impl<T> {where T:'a} :
	(T) <-> (&'a T)
}

new_struct_func!{
	pub MapFromRef
	impl<'a,T>:
	(&'a T) -> (T)
}

new_struct_func!{
	pub MapMut<'a>
	impl<T> {where T:'a}:
	(T) <-> (&'a mut T)
}

new_struct_func!{
	pub SetMut
	impl<'a,T>:
	((&'a mut T,T)) -> (())
	|i|{*i.0=i.1;}
}

new_struct_func!{
	pub FoldVecPush
	impl<VD,T> {where VD:Deref<Target = Vec<T>>+DerefMut}:
	((VD,T)) -> (VD)
	|mut i|{
		i.0.push(i.1);
		i.0
	}
}

/// `<HList as HMappable<Mapper>>::Output`
/// 
/// [HMappable]
pub type HMap<HList,Mapper>=<HList as HMappable<Mapper>>::Output;
/// `<HList as HMappable<Poly<Mapper>>>::Output`
/// 
/// [HMappable]
pub type HMapP<HList,Mapper>=<HList as HMappable<Poly<Mapper>>>::Output;
/// `<A as HZippable<B>>::Zipped`
/// 
/// [HZippable]
pub type HZip<A,B>=<A as HZippable<B>>::Zipped;
/// `<T as ToRef<'a>>::Output`
/// 
/// [`ToRef`]
pub type HToRef<'a,T>=<T as ToRef<'a>>::Output;
/// `<T as ToMut<'a>>::Output`
/// 
/// [`ToMut`]
pub type HToMut<'a,T>=<T as ToMut<'a>>::Output;
/// `<A as Add<B>>::Output`
/// 
/// [Add]
pub type Sum<A,B>=<A as Add<B>>::Output;

/// map type `HList` by [TypeFunc] `TypeFn`
/// 
/// `HList` -> [MapToPhantom] -> `TypeFunc` -> [MapFromPhantomPanic]
/// 
/// `HMapP<HMapP<HMapP<HList,MapToPhantom>,TypeFn>,MapPhantomType>`
pub type HTypeMapP<HList,TypeFunc> = HMapP<HMapP<HMapP<HList,MapToPhantom>,TypeFunc>,MapFromPhantomDataPanic>;

/// Convert `TypeFn` from [TypeFunc] to [Func] that can be used in [HMapP]
/// 
/// PANICS WHEN USED IN [frunk::HCons::map] AS `mapper`
/// 
/// [MapToPhantom] * `TypeFn` * [MapFromPhantomPanic]
/// 
/// `ChainFunc<ChainFunc<MapToPhantom,TypeFn>,MapPhantomType>`
pub type HTypeFnToMapper<TypeFn> = ChainFunc<ChainFunc<MapToPhantom,TypeFnAsPhantomFn<TypeFn>>,MapFromPhantomDataPanic>;

/// repeat `Input` to get `Self`
/// 
/// see [`HRepeat`] [`h_repeat`] for usage
pub trait HRepeatFrom<Input> {
	/// repeat `Input` to get `Self`
	fn output_repeat(v:Input)->Self;
} 

impl<T> HRepeatFrom<T> for HNil {
	fn output_repeat(_v:T)->Self {
		HNil
	}
}

impl<V,T> HRepeatFrom<V> for HCons<V,T>
	where T:HRepeatFrom<V>,
	V:Clone
{
	fn output_repeat(v:V)->Self {
		HCons { head: v.clone(), tail: T::output_repeat(v) }
	}
}

/// a weird way to infer a type (H) that [`HRepeatFrom`]<T>
/// 
/// see [`HRepeat`] [`h_repeat`]

pub struct HInferRepeatFromStruct<H,T>(PhantomData<(H,T)>)
where H:HRepeatFrom<T>
;

impl_phantom!(HInferRepeatFromStruct<H,T> where H:HRepeatFrom<T>);

// impl<H, T> Default for HInferRepeatFromStruct<H, T>
// where H:HRepeatFrom<T>
// {
//     fn default() -> Self {
// 		Self(Default::default())
// 	}
// }

// impl<H, T> Clone for HInferRepeatFromStruct<H, T>
// where H:HRepeatFrom<T>
// {
//     fn clone(&self) -> Self {
// 		Self(self.0.clone())
// 	}
// }

// impl<H, T> Copy for HInferRepeatFromStruct<H, T>
// where H:HRepeatFrom<T>
// {
// }

/// a weird trait to infer a type (H) that [`HRepeatFrom`]<T>
/// 
/// see [`HRepeat`] [`h_repeat`]
pub trait HInferRepeatFromTrait {
	/// inferred type that [`HRepeatFrom`]<T>
	type Repeated;
}

impl<H,T> HInferRepeatFromTrait for HInferRepeatFromStruct<H,T>
where H:HRepeatFrom<T>
{
	type Repeated=H;
}

/// `HRepeat<_,T>` gets a type that filled with T, and length inferred fron usage
/**
# Example
```    
# use frunk::hlist;
# use wacky_bag_hlist::h_list_helpers::HRepeat;
# use wacky_bag_hlist::h_h_zippable::HHZippable;
assert_eq!(
	hlist![1,2.0,"3"].h_zip(
		// <HRepeat<_,HNil> as Default>::default()
		h_repeat(HNil)
	),
	hlist![HCons{head:1,tail:HNil},HCons{head:2.0,tail:HNil},hlist!("3")]
)
```
 */
pub type HRepeat<H,V>=<HInferRepeatFromStruct<H, V> as HInferRepeatFromTrait>::Repeated;

/// repeat v
pub fn h_repeat<H,V>(v:V)->HRepeat<H,V>
	where H:HRepeatFrom<V>
{
	H::output_repeat(v)
}

new_struct_func!(pub MapToHList impl <T> : (T) <-> (HList!(T)) |i|HCons{head:i,tail:HNil},|h|h.head );

new_struct_func!{pub MapClone2 < 'a > impl < T > {where T:'a+Clone} : (&'a T) <-> (T) |i|i.clone()}


#[cfg(test)]
mod test{
    use frunk::hlist;

	use crate::h_h_zippable::HHZippable;

	use super::*;

	#[test]
	fn test() {
		assert_eq!(
			hlist![1,2.0,"3"].h_zip(
				// <HRepeat<_,HNil> as Default>::default()
				h_repeat(HNil)
			),
			hlist![HCons{head:1,tail:HNil},HCons{head:2.0,tail:HNil},hlist!("3")]
		)
	}
}