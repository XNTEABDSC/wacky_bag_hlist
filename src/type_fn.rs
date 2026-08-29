//! [`TypeFunc`] [`BijectiveTypeFunc`] [`BijectiveFunc`] and macros to implement them easily

// //! [`TypeFunc`]
// //! [`BijectiveTypeFunc`]
// //! [`BijectiveFunc`]
// //! [`ReverseFunc`]
// //! [`FuncAsTypeFunc`]
// //! [`TypeFnAsPhantomFn`]
// //! [`impl_func`]
// //! [`new_struct_func`]
// //! [`impl_func_closure`]
// //! [`new_new_type_func`]
// //! [`new_new_phantom_type_func`]

use std::{any::type_name, marker::PhantomData};

use frunk::Func;

/// You can consider that this is a function about type
pub trait TypeFunc<Input>{
	/// `Output` type
	type Output;
}

// causes conflicting implementations
// impl<F,I,O> TypeFunc<I> for F
// 	where F:Func<I,Output = O>
// {
// 	type Output=O;
// }

// pub trait TypeFuncRev<Output> {
// 	type Input;
// }

// you will find BijectiveTypeFunc<Input,Output> useless as a trait bound that requires both Input and Output
// pub trait BijectiveTypeFunc<Input,Output> : TypeFunc<Input,Output = Output>+TypeFuncRev<Output,Input = Input> {
// }

/// Shows how to find Input via Output
pub trait BijectiveTypeFunc<Output> : TypeFunc<Self::Input,Output = Output> {
	/// `Input` type
	type Input;
}

// causes conflicting implementations
// impl<F,I,O> BijectiveTypeFunc<O> for F
// 	where F: Func<I,Output = O> + BijectiveFunc<O,Input = I> + TypeFunc<I,Output = O>
// {
// 	type Input=I;
// }

/// Shows how to find Input via Output
pub trait BijectiveFunc<Output> : Func<Self::Input,Output = Output> {
	/// `Input` type
	type Input;
	/// from `Output` to `Input`
	fn inv_call(output:Output)->Self::Input;
}

/// use [`Func`] and [`BijectiveFunc`] as [`TypeFunc`] and [`BijectiveTypeFunc`]
pub struct FuncAsTypeFunc<F>(pub F);

impl<T,I,O> TypeFunc<I> for FuncAsTypeFunc<T>
	where T:Func<I,Output = O>
{
	type Output=O;
}

impl<T,I,O> BijectiveTypeFunc<O> for FuncAsTypeFunc<T>
	where T:BijectiveFunc<O,Input = I>
{
	type Input=I;
}

impl<T,I,O> Func<I> for FuncAsTypeFunc<T>
	where T:Func<I,Output = O>
{
	type Output=O;

	fn call(i: I) -> Self::Output {
		T::call(i)
	}
}

impl<T,I,O> BijectiveFunc<O> for FuncAsTypeFunc<T>
	where T:BijectiveFunc<O,Input = I>
{
	type Input = I;
	
	fn inv_call(output:O)->Self::Input {
		T::inv_call(output)
	}
	
}

/// converts [TypeFunc] into [Func] that uses [PhantomData] as input and output
/// 
/// useful in type expression that can't provide implementation, while can be used by [super::h_list_helpers::HMapP]
pub struct TypeFnAsPhantomFn<F>(pub F);

impl<T,I,O> Func<PhantomData<I>> for TypeFnAsPhantomFn<T>
	where T:TypeFunc<I,Output = O>
{
	type Output=PhantomData<O>;

	fn call(_: PhantomData<I>) -> Self::Output {
		Default::default() 
	}
}

impl<T,I,O> BijectiveFunc<PhantomData<O>> for TypeFnAsPhantomFn<T>
	where T:BijectiveTypeFunc<O,Input = I>
{
	type Input=PhantomData<I>;

	fn inv_call(_output:PhantomData<O>)->Self::Input {
		Default::default() 
	}
}

// crate::new_struct_func!{
// 	#[doc= "`PhantomData<T>` to `T` and should only be used as type wrapping.\n "]
// 	pub MapFromPhantomPanic
// 	impl<T>:
// 	(PhantomData<T>) <-> (T)
// 	|_i|panic!("{:?} should not be called, but only used in type expression, cant get {} from PhantomData<{}>",MapFromPhantomPanic,type_name::<T>(),type_name::<T>()),
// 	|_i|Default::default()
// }

/// `PhantomData<T>` to `T` and should only be used as type wrapping.
/// 
/// impl [Func] is provided only to make use of hlist functions type expression but NOT to be called.
/// 
/// # Panic
/// 
/// Panics when used e.g. in [frunk::HCons::map] as `mapper`
#[derive(Debug)]
pub struct MapFromPhantomDataPanic(());

impl<T> Func<PhantomData<T>> for MapFromPhantomDataPanic {
	type Output=T;

	fn call(_i: PhantomData<T>) -> Self::Output {
		panic!("{:?} should not be called, but only used in type expression, cant get {} from PhantomData<{}>",MapFromPhantomDataPanic(()),type_name::<T>(),type_name::<T>())
	}
}

impl<T> BijectiveFunc<T> for MapFromPhantomDataPanic {
	type Input=PhantomData<T>;

	fn inv_call(_output:T)->Self::Input {
		Default::default()
	}
}
/// [`MapFromPhantomPanic`]
pub type MapPhantomType=MapFromPhantomDataPanic;

/**
impl [`TypeFunc`] [`BijectiveTypeFunc`] [`Func`] [`BijectiveFunc`] by a easy way.

see [`new_struct_func`]
*/
#[macro_export]
macro_rules! impl_func {
( $(< $($g:tt),+ >)? for $name:ty $({ where $($where:tt)+ })? : ($a:ty) <-> ($b:ty) $a2b:expr) => {
	$crate::impl_func!{$(< $($g),+ >)? for $name $({where $($where)+})? : ($a) <-> ($b)}
	impl $(< $($g),+ >)? frunk::traits::Func<$a> for $name
	$(where $($where)+)?
	{
		type Output=$b;
		fn call(i:$a)->Self::Output{
			let f=$crate::utils::restrict_fn_once_type::<_,$a,$b>($a2b);// WHAT HAPPENED WHY ITS NOT REPLACED
			f(i)
		}
	}
};
($(< $($g:tt),+ >)?for $name:ty $({ where $($where:tt)+ })? : ($a:ty) <-> ($b:ty) ) => {
	$crate::impl_func!{$(< $($g),+ >)? for $name $({where $($where)+})? : ($a) -> ($b)}
	impl$(< $($g),+ >)? $crate::type_fn::BijectiveTypeFunc<$b> for $name
	$(where $($where)+)?
	{
		type Input=$a;
	}
};
($(< $($g:tt),+ >)? for $name:ty $({ where $($where:tt)+ })? : ($a:ty) <-> ($b:ty) $a2b:expr,$b2a:expr) => {
	$crate::impl_func!{$(< $($g),+ >)? for $name $({where $($where)+})? : ($a) <-> ($b) $a2b}
	impl$(< $($g),+ >)? $crate::type_fn::BijectiveFunc<$b> for $name
	$(where $($where)+)?
	{
		type Input=$a;
		fn inv_call(o:$b)->Self::Input{
			let f=$crate::utils::restrict_fn_once_type::<_,$b,$a>($b2a);
			f(o)
		}
	}
};
($(< $($g:tt),+ >)? for $name:ty $({ where $($where:tt)+ })? : ($a:ty) -> ($b:ty) $a2b:expr) => {
	$crate::impl_func!{$(< $($g),+ >)? for $name $({where $($where)+})? : ($a) -> ($b)}
	impl$(< $($g),+ >)? frunk::traits::Func<$a> for $name
	$(where $($where)+)?
	{
		type Output=$b;
		fn call(i:$a)->Self::Output{
			let f=$crate::utils::restrict_fn_once_type::<_,$a,$b>($a2b);// WHAT HAPPENED WHY ITS NOT REPLACED
			f(i)
		}
	}
};
($(< $($g:tt),+ >)? for $name:ty $({ where $($where:tt)+ })? : ($a:ty) $a2b:expr) => {
	$crate::impl_func!{$(< $($g),+ >)? for $name $({where $($where)+})? : ($a) -> (()) $a2b}
};
($(< $($g:tt),+ >)? for $name:ty $({ where $($where:tt)+ })? : ($a:ty) -> ($b:ty) ) => {
	impl$(< $($g),+ >)? $crate::type_fn::TypeFunc<$a> for $name
	$(where $($where)+)?
	{
		type Output=$b;
	}
};
}

/**
Make a new type that impl [`Func`] [`BijectiveFunc`] [`TypeFunc`] [`BijectiveTypeFunc`]

# Example
```
# use wacky_bag_hlist::new_struct_func;
# use std::marker::PhantomData;
new_struct_func!(
	pub MapToPhantom // name, can have <T>
	impl<T> // impl with generic parameter, can add where clause inside a {}
	:
	(T) <-> (PhantomData<T>) // input output type, -> is one dir (injective), <-> is bijective
	|_i|Default::default() // FnOnce(T)->PhantomData<T>
	// |_p|panic!() // BijectiveFunc if you want
);
```
another example
```
# use std::iter::Chain;
# use wacky_bag_hlist::new_struct_func;
new_struct_func!(
	pub FoldChainIter impl <Acc,X,Item>
	{where Acc:Iterator<Item = Item>,
		X:Iterator<Item = Item>}
	:
	((Acc,X)) <-> (Chain<Acc,X>)
	|i|i.0.chain(i.1)
);
```

*/
#[macro_export]
macro_rules! new_struct_func {

($(#[$meta:meta])* $vis:vis $name:ident < $($tt:tt),* >  impl $({ where $($where:tt)+ })? : $($then:tt)* ) => {
	$(#[$meta])*

	#[doc = 
	concat!(
		"`",
		stringify!( <$($tt),*> for $name<$($tt),*>

		$({where $($where)+})?: 

		$($then)* ),
		"`"
	)
	]

	$vis struct $name < $($tt),* > (pub PhantomData< $crate::phantom_data_type_params!( $($tt),* ) >);

	$crate::impl_phantom!{ $name < $($tt),* >}

	$crate::impl_func!{ <  $($tt),* > for $name< $($tt),* > $({where $($where)+})?: $($then)* }
};

($(#[$meta:meta])* $vis:vis $name:ident impl $(< $($tt:tt),* >)? $( { where $($where:tt)+ })?: $($then:tt)* ) => {
	$(#[$meta])*

	#[doc = concat!(
		"`",
		stringify!($(<$($tt),*>)? for $name 
		
		$({where $($where)+})?: 
		
		$($then)*),
		"`"
	)]
	
	#[derive(Debug,Default,Clone,Copy)]
	$vis struct $name;

	$crate::impl_func!{ $(< $($tt),* >)? for $name $({where $($where)+})?: $($then)* }
};

($(#[$meta:meta])* $vis:vis $name:ident < $($sglt:lifetime,)* $($sgty:ident),* >  impl < $($iglt:lifetime,)* $($igty:ident),* > $( { where $($where:tt)+ })?: $($then:tt)* ) => {
	$(#[$meta])*

	#[doc = concat!(
		"`",
		stringify!(<$($sglt,)* $($sgty,)*  $($iglt,)* $($igty),*> for $name <$($sglt),*$($sgty),*>

		$({where $($where)+})?: 

		$($then)*),
		"`"
	)]

	$vis struct $name <$($sglt,)* $($sgty),*> (pub PhantomData< $crate::phantom_data_type_params!( $($sglt),*$($sgty),* ) >);

	$crate::impl_phantom!{ $name <$($sglt),*$($sgty),*>}

	$crate::impl_func!{ <  $($sglt,)* $($sgty,)* $($iglt,)* $($igty),* > for $name< $($sglt,)* $($sgty),* > $({where $($where)+})?: $($then)* }
};

($(#[$meta:meta])* $vis:vis $name:ident < $($sglt:lifetime),* >  $($then:tt)* ) =>{
	$crate::new_struct_func!($(#[$meta])* $vis $name < $($sglt,)* >  $($then)*);
};

($(#[$meta:meta])* $vis:vis $name:ident < $($sglt:lifetime,)* $($sgty:ident),* >  impl < $($iglt:lifetime),* > $($then:tt)* ) =>{
	$crate::new_struct_func!($(#[$meta])* $vis $name < $($sglt,)* $($sgty),* >  impl < $($iglt,)* > $($then)*);
};
}


/**
Nameless [`new_struct_func`]

# Example
```
# use wacky_bag_hlist::impl_func_closure;
# use frunk::{Poly, hlist};
# use std::ops::Add;
let func=impl_func_closure!(<T>{where T:Add<T>+Clone}: (T) -> (<T as Add<T>>::Output) |i|i.clone()+i);
assert_eq!(
	hlist![1,2.0,3usize].map(Poly(func)),
	hlist![2,4.0,6usize]
);
```
 */
#[macro_export]
macro_rules! impl_func_closure {
($($tt:tt)* ) => {
	{
		$crate::new_struct_func!(ImplFuncClause impl $($tt)*);
		ImplFuncClause
	}
};
}

/// [`new_struct_func`] for `T` <-> `pub NewType<T>(pub T);`
/// 
/// # Example
/**
```
# use wacky_bag_hlist::impl_func_closure;
pub struct NewType<T>(pub T);
new_new_type_func!( NewType pub MapNewType );
```
*/
/// Also able to impl `&NewType<T>` -> `&T`, `&mut NewType<T>` -> `&mut T`, 
#[macro_export]
macro_rules! new_new_type_func {
	($nt_name:ident 
		$($once_fn_name_vis:vis $once_fn_name:ident)? 
		$(ref $ref_fn_name_vis:vis $ref_fn_name:ident)?
		$(mut $mut_fn_name_vis:vis $mut_fn_name:ident)?
	) => {
		$(
			$crate::new_struct_func!{
				$once_fn_name_vis $once_fn_name
				impl<T>:
				(T) <-> ($nt_name<T>)
				|i|$nt_name(i),
				|i|i.0
			}
		)?
		$(
			$crate::new_struct_func!{
				$ref_fn_name_vis $ref_fn_name
				impl<'a,T>:
				(&'a $nt_name<T>) <-> (&'a T)
				|i|&i.0
			}
		)?
		$(
			$crate::new_struct_func!{
				$mut_fn_name_vis $mut_fn_name
				impl<'a,T>:
				(&'a mut $nt_name<T>) <-> (&'a mut T)
				|i|&mut i.0
			}
		)?
	};
}
/// [`new_struct_func`] for `pub Name<T>(PhantomData<T>);`
#[macro_export]
macro_rules! new_new_phantom_type_func {
	($nt_name:ident $vis:vis $fn_name:ident ) => {
		$crate::new_struct_func!(
			$vis $fn_name
			impl<T>:
			(T)<->($nt_name<T>)
			|_|Default::default()
		);
	};
}

