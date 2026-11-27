//! Some helper things for h_list, including some impl [`Func`] and some type expression (type alias)

/*!
[`MapToPhantom`]
[`FoldChainIter`]
[`MapDeref`]
[`MapDerefMut`]
[`TakeDerefMap`]
[`TakeDerefMutMap`]
[`MapDerefT`]
[`MapClone`]
[`MapMutToRef`]
[`MapNeg`]
[`MapNegRev`]
[`MapRef`]
[`MapFromRef`]
[`MapMut`]
[`MapFromMut`]
[`SetMut`]
[`FoldVecPush`]
[`HMap`]
[`HMapP`]
[`HZip`]
[`HToRef`]
[`HToMut`]
[`Sum`]
[`HTypeMapP`]
[`HTypeFnToMapper`]
[`HRepeatFrom`]
[`MapToHList`]
[`MapClone2`]
*/

use std::{iter::Chain, marker::PhantomData, ops::*};

use frunk::{
    Func, HCons, HList, HNil, Poly, ToMut, ToRef, hlist::{HFoldLeftable, HFoldRightable, HMappable, HZippable},
};

use crate::{
    chain_fn::ChainFunc, impl_phantom, new_struct_func, type_fn::{BijectiveFunc, BijectiveTypeFunc, MapFromPhantomDataPanic, TypeFnAsPhantomFn, TypeFunc},
};

new_struct_func!(
    pub MapToPhantom
    impl<T>:
    (T) <-> (PhantomData<T>)
    |_i|Default::default()
);

new_struct_func!(
    pub FIdent
    impl<T>:
    (T)<->(T)
    |a|a,
    |a|a
);

new_struct_func!(
    pub FoldChainIter impl <Acc,X,Item>
    {where Acc:Iterator<Item = Item>,
        X:Iterator<Item = Item>}
    :
    ((Acc,X)) <-> (Chain<Acc,X>)
    |i|i.0.chain(i.1)
);

new_struct_func! {
    pub MapDeref
    impl <'a,TA,TB>
    {where TA:Deref<Target=TB>+'a,TB:'a}:
    (&'a TA) -> (&'a TB)
    |i|i.deref()
}

new_struct_func! {
    pub MapDerefMut
    impl <'a,TA,TB>
    {where TA:'a+Deref<Target=TB>+DerefMut,TB:'a}:
    (&'a mut TA) -> (&'a mut TB)
    |i|i.deref_mut()
}

new_struct_func! {
    pub MapClone
    impl <'a,T> {where T:Clone+'a}:
    (&'a T) -> (T)
    |i|i.clone()
}

new_struct_func! {
    pub MapMutToRef
    impl<'a,T> {where T:'a}:
    (&'a mut T) <-> (&'a T)
    |i|i
}

/// a:A -> b:B=-a
/// 
/// b:B -> a:A=-b
#[derive(Debug, Default, Clone, Copy)]
pub struct MapNeg;

impl<T1, T2> TypeFunc<T1> for MapNeg
where
    T1: Neg<Output = T2>,
{
    type Output = T2;
}
impl<T1, T2> BijectiveTypeFunc<T2> for MapNeg
where
    T1: Neg<Output = T2>,
    T2: Neg<Output = T1>,
{
    type Input = T1;
}
impl<T1, T2> Func<T1> for MapNeg
where
    T1: Neg<Output = T2>,
{
    type Output = T2;
    fn call(i: T1) -> Self::Output {
        -i
    }
}
impl<T1, T2> BijectiveFunc<T2> for MapNeg
where
    T1: Neg<Output = T2>,
    T2: Neg<Output = T1>,
{
    type Input = T1;
    fn inv_call(o: T2) -> Self::Input {
        -o
    }
}

new_struct_func! {
    pub MapRef<'a>
    impl<T> {where T:'a} :
    (T) <-> (&'a T)
}

new_struct_func! {
    pub MapFromRef
    impl<'a,T>:
    (&'a T) -> (T)
}

new_struct_func! {
    pub MapMut<'a>
    impl<T> {where T:'a}:
    (T) <-> (&'a mut T)
}

new_struct_func! {
    pub MapFromMut
    impl<'a,T>:
    (&'a mut T) -> (T)
}

new_struct_func! {
    pub SetMut
    impl<'a,T>:
    ((&'a mut T,T)) -> (())
    |i|{*i.0=i.1;}
}

new_struct_func! {
    pub FoldVecPush
    impl<VD,T> {where VD:Deref<Target = Vec<T>>+DerefMut}:
    ((VD,T)) -> (VD)
    |mut i|{
        i.0.push(i.1);
        i.0
    }
}

macro_rules! new_struct_func_op_assign {
	($sname:ident,$tname:ident,$op:tt) => {
crate::new_struct_func!(
	pub $sname
	impl<'a,A,B> {where A:$tname<B>}:
	((&'a mut A,B)) -> (())
	|(a,b)|{*a $op b}
);
	};
}

macro_rules! new_struct_func_op {
	($sname:ident,$tname:ident,$op:tt) => {
crate::new_struct_func!(
	pub $sname
	impl<A,B,C> {where A:$tname<B,Output=C>}:
	((A,B)) -> (C)
	|(a,b)|{a $op b}
);
	};
}

new_struct_func_op_assign!(MapAddAssign,AddAssign,+=);
new_struct_func_op_assign!(MapSubAssign,SubAssign,-=);
new_struct_func_op_assign!(MapMulAssign,MulAssign,*=);
new_struct_func_op_assign!(MapDivAssign,DivAssign,/=);
new_struct_func_op_assign!(MapRemAssign,RemAssign,%=);
new_struct_func_op!(MapAdd,Add,+);
new_struct_func_op!(MapSub,Sub,-);
new_struct_func_op!(MapMul,Mul,*);
new_struct_func_op!(MapDiv,Div,/);
new_struct_func_op!(MapRem,Rem,%);
new_struct_func_op_assign!(MapBitAndAssign, BitAndAssign, &=);
new_struct_func_op_assign!(MapBitOrAssign,  BitOrAssign,  |=);
new_struct_func_op_assign!(MapBitXorAssign, BitXorAssign, ^=);
new_struct_func_op!(MapBitAnd, BitAnd, &);
new_struct_func_op!(MapBitOr,  BitOr,  |);
new_struct_func_op!(MapBitXor, BitXor, ^);

macro_rules! new_struct_func_op_self_fixed_output {
	($sname:ident,$tname:ident,$op:tt,$output:ty) => {
crate::new_struct_func!(
	pub $sname
	impl<'a,A> {where A:$tname}:
	((&'a A,&'a A)) -> ($output)
	|(a,b)|{a $op b}
);
	};
}

new_struct_func_op_self_fixed_output!(MapEq, Eq, ==, bool);
new_struct_func_op_self_fixed_output!(MapPartialEq, PartialEq, ==, bool);
new_struct_func_op_self_fixed_output!(MapPartialNe, PartialEq, !=, bool);
new_struct_func_op_self_fixed_output!(MapPartialOrdLT, PartialOrd, <, bool);
new_struct_func_op_self_fixed_output!(MapPartialOrdLE, PartialOrd, <=, bool);
new_struct_func_op_self_fixed_output!(MapPartialOrdGT, PartialOrd, >, bool);
new_struct_func_op_self_fixed_output!(MapPartialOrdGE, PartialOrd, >=, bool);

macro_rules! new_struct_func_op_bool {
	($sname:ident,$op:tt) => {
crate::new_struct_func!(
	pub $sname
	impl:
	((bool,bool)) -> (bool)
	|(a,b)|{a $op b}
);
	};
}
new_struct_func_op_bool!(MapBoolAnd,&&);
new_struct_func_op_bool!(MapBoolOr,||);
new_struct_func_op_bool!(MapBoolXor,^);

// new_struct_func!(
// 	pub MapAdd
// 	impl<A,B,C> {where A:Add<B,Output = C>}:
// 	((A,B)) -> (c)
// 	|(a,b)|{a + b}
// );

// new_struct_func!(
// 	pub MapAddAssign
// 	impl<'a,A,B> {where A:AddAssign<B>}:
// 	((&'a mut A,B)) -> (())
// 	|(a,b)|{*a+=b}
// );

/// `<HList as HMappable<Mapper>>::Output`
///
/// [HMappable]
pub type HMap<HList, Mapper> = <HList as HMappable<Mapper>>::Output;
/// `<HList as HMappable<Poly<Mapper>>>::Output`
///
/// [HMappable]
pub type HMapP<HList, Mapper> = <HList as HMappable<Poly<Mapper>>>::Output;
/// `<A as HZippable<B>>::Zipped`
///
/// [HZippable]
pub type HZip<A, B> = <A as HZippable<B>>::Zipped;
/// `<T as ToRef<'a>>::Output`
///
/// [`ToRef`]
pub type HToRef<'a, T> = <T as ToRef<'a>>::Output;
/// `<T as ToMut<'a>>::Output`
///
/// [`ToMut`]
pub type HToMut<'a, T> = <T as ToMut<'a>>::Output;

new_struct_func!(
    pub FToRef
    impl<'a,T>{where T:ToRef<'a>}:
    (&'a T)->(T::Output)
    |v|v.to_ref()
);

new_struct_func!(
    pub FToMut
    impl<'a,T>{where T:ToMut<'a>}:
    (&'a mut T)->(T::Output)
    |v|v.to_mut()
);

/// `<A as Add<B>>::Output`
///
/// [Add]
pub type Sum<A, B> = <A as Add<B>>::Output;

/// map type `HList` by [TypeFunc] `TypeFn`
///
/// `HList` -> [MapToPhantom] -> `TypeFunc` -> [MapFromPhantomPanic]
///
/// `HMapP<HMapP<HMapP<HList,MapToPhantom>,TypeFn>,MapPhantomType>`
pub type HTypeMapP<HList, TypeFunc> = HMapP<HList, HTypeFnToMapper<TypeFunc>>;
// HMapP<HMapP<HMapP<HList,MapToPhantom>,TypeFunc>,MapFromPhantomDataPanic>; use HTypeFnToMapper instead of 3 HMapP, or else more where is required

/// Convert `TypeFn` from [TypeFunc] to [Func] that can be used in [HMapP]
///
/// # Panic
///
/// Panic when used in [frunk::HCons::map] as `mapper`
///
/// [MapToPhantom] * `TypeFn` * [MapFromPhantomPanic]
///
/// `ChainFunc<ChainFunc<MapToPhantom,TypeFn>,MapPhantomType>`
pub type HTypeFnToMapper<TypeFn> =
    // HChainFunc<HList!(MapToPhantom,TypeFnAsPhantomFn<TypeFn>,MapFromPhantomDataPanic)>;
    ChainFunc<ChainFunc<MapToPhantom, TypeFnAsPhantomFn<TypeFn>>, MapFromPhantomDataPanic>;

/// repeat `Input` to get `Self`
///
/// see [`HRepeat`] [`h_repeat`] for usage
pub trait HRepeatFrom<Input> {
    /// repeat `Input` to get `Self`
    fn output_repeat(v: Input) -> Self;
}

impl<T> HRepeatFrom<T> for HNil {
    fn output_repeat(_v: T) -> Self {
        HNil
    }
}

impl<V, T> HRepeatFrom<V> for HCons<V, T>
where
    T: HRepeatFrom<V>,
    V: Clone,
{
    fn output_repeat(v: V) -> Self {
        HCons {
            head: v.clone(),
            tail: T::output_repeat(v),
        }
    }
}

/// a weird way to infer a type (H) that [`HRepeatFrom`]<T>
///
/// see [`HRepeat`] [`h_repeat`]

pub struct HInferRepeatFromStruct<H, T>(PhantomData<(H, T)>)
where
    H: HRepeatFrom<T>;

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

impl<H, T> HInferRepeatFromTrait for HInferRepeatFromStruct<H, T>
where
    H: HRepeatFrom<T>,
{
    type Repeated = H;
}

/// `HRepeat<_,T>` gets a type that filled with T, and length inferred fron usage
/**
# Example
```
# use frunk::hlist;
# use wacky_bag_hlist::h_list_helpers::HRepeat;
assert_eq!(
  hlist![1, 2.0, "3"].zip(
    // <HRepeat<_,HNil> as Default>::default()
    h_repeat("spam")
  ),
  hlist![
    (1,"spam"),
    (2.0,"spam"),
    ("3","spam")
  ]
)
```
 */
pub type HRepeat<H, V> = <HInferRepeatFromStruct<H, V> as HInferRepeatFromTrait>::Repeated;

/// repeat v
pub fn h_repeat<H, V>(v: V) -> HRepeat<H, V>
where
    H: HRepeatFrom<V>,
{
    H::output_repeat(v)
}

new_struct_func!(pub MapToHList impl <T> : (T) <-> (HList!(T)) |i|HCons{head:i,tail:HNil},|h|h.head );

new_struct_func!(pub MapHCons impl <A,B> : ((A,B)) <-> (HCons<A,B>) |(a,b)|HCons{head:a,tail:b});

new_struct_func! {pub MapClone2 < 'a > impl < T > {where T:'a+Clone} : (&'a T) <-> (T) |i|i.clone()}

new_struct_func! {pub FoldApply impl<F,I,O> {where F:Func<I,Output=O>}: ( (I,F) )->(O) |(i,_f)|F::call(i)}

/// <HMapP<H,MapToPhantom> as Default>::default()
pub fn h_phantom<H>()
->HMapP<H,MapToPhantom>
where H:HMappable<Poly<MapToPhantom>, Output :Default>
{
    <HMapP<H,MapToPhantom> as Default>::default()
}

/// [HFoldLeftable]
#[derive(Debug,Default,Clone, Copy)]
pub struct FHFoldLeft<F>(pub F);

impl<F,I,Acc,O> Func<(Acc,I)> for FHFoldLeft<F>
where I:HFoldLeftable<F,Acc,Output = O>,
F:Default
{
    type Output=O;

    fn call((acc,i): (Acc,I)) -> Self::Output {
        i.foldl(F::default(), acc)
    }
}

impl<F,I,Acc,O> TypeFunc<(Acc,I)> for FHFoldLeft<F>
where I:HFoldLeftable<F,Acc,Output = O>,
F:Default
{
    type Output=O;
}

/// [HFoldRightable]
#[derive(Debug,Default,Clone, Copy)]
pub struct FHFoldRight<F>(pub F);

impl<F,I,Acc,O> Func<(Acc,I)> for FHFoldRight<F>
where I:HFoldRightable<F,Acc,Output = O>,
F:Default
{
    type Output=O;

    fn call((acc,i): (Acc,I)) -> Self::Output {
        i.foldr(F::default(), acc)
    }
}

impl<F,I,Acc,O> TypeFunc<(Acc,I)> for FHFoldRight<F>
where I:HFoldRightable<F,Acc,Output = O>,
F:Default
{
    type Output=O;
}

new_struct_func!(
    pub FTuple2Swap
    impl<A,B>:
    ((A,B))<->((B,A))
    |(a,b)|{(b,a)},
    |(a,b)|{(b,a)}
);
new_struct_func!(
    pub FTuple2Sel1
    impl<A,B>:
    ((A,B))->(A)
    |(a,_b)|{a}
);
new_struct_func!(
    pub FTuple2Sel2
    impl<A,B>:
    ((A,B))->(B)
    |(_a,b)|{b}
);
new_struct_func!(
    pub FTuple2With1<A>
    impl<B>:
    (B)->((A,B))
);
new_struct_func!(
    pub FTuple2With2<B>
    impl<A>:
    (A)->((A,B))
);

new_struct_func!(
    pub FHZip
    impl<A,B,O> {where A:HZippable<B,Zipped=O>}:
    ((A,B))->(O)
    |(a,b)|{a.zip(b)}
);

new_struct_func!(
    pub FControlFlowTypes
    impl<B,C>:
    (ControlFlow<B,C>)<->((B,C))
);

/// [`HMappable`]
#[derive(Debug,Default,Clone, Copy)]
pub struct FHMap<F>(pub F);
impl<F:Default,I,O> Func<I> for FHMap<F>
where I:HMappable<F,Output = O>
{
    type Output=O;

    fn call(i: I) -> Self::Output {
        i.map(F::default())
    }
}

/// `HList![T...]:HFoldRightable< Poly<FFoldCloneZip>,B ,Output = HList![(T,B)...]>`
pub struct FFoldCloneZip;

impl<HL,B,T> TypeFunc< ((HL,B),T) > for FFoldCloneZip
{
    type Output = ( HCons<(T,B),HL> ,B);
    
}

impl<HL,B,T> Func< ((HL,B),T) > for FFoldCloneZip
where B:Clone
{
    type Output = ( HCons<(T,B),HL> ,B);
    
    fn call(((hl,b),t): ((HL,B),T)) -> Self::Output {
        (
            HCons{
                head:(t,b.clone()),
                tail:hl
            },
            b
        )
    }
}


#[cfg(test)]
mod test {
    use frunk::hlist;

    use super::*;

    #[test]
    fn test_repeat() {
        assert_eq!(
            hlist![1, 2.0, "3"].zip(
                // <HRepeat<_,HNil> as Default>::default()
                h_repeat("spam")
            ),
            hlist![
                (1,"spam"),
                (2.0,"spam"),
                ("3","spam")
            ]
        )
    }
}
