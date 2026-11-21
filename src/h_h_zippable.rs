//! Zip 2 hlist into 1 hlist with hlist inside.
//! 
//! Transpose a hlist hlist like a matrix.
//! 
//! [`HHZippable`]
//! [`HTranspose`]

use std::process::Output;

use frunk::{HCons, HList, HNil, Poly, hlist::{self, HFoldRightable, HMappable, HZippable, foldr_owned::HFoldRightableOwned}};

use crate::{chain_fn::ChainFunc, h_chain_fn::HChainFunc, h_list_helpers::{FHMap, FHZip, FTuple2Swap, HMapP, HRepeat, HRepeatFrom, HZip, MapHCons, MapToHList, h_repeat}};
/// zip 2 hlist by HCons
pub trait HHZippable<Other> {
	/// result type
    type HZipped;
	/// zip 2 hlist by HCons
	/// 
	/** 
	```
	assert_eq!(
		hlist![1,2.0,"3"].h_zip(
		hlist![hlist!["4"],hlist![5],hlist![6.0]]
		)
		,
		hlist![
			hlist![1,"4"],
			hlist![2.0,5],
			hlist!["3",6.0]
		]
	);
	```
	*/
    fn h_zip(self, other: Other) -> Self::HZipped;
}

// impl HHZippable<HNil> for HNil {
// 	type HZipped=HNil;

// 	fn h_zip(self, _other: HNil) -> Self::HZipped {
// 		HNil
// 	}
// }

// impl<H1,T1,H2,T2> HHZippable<HCons<H2,T2>> for HCons<H1,T1>
// 	where T1:HHZippable<T2>
// {
// 	type HZipped=HCons<
// 		HCons<H1,H2>,
// 		T1::HZipped
// 	>;

// 	fn h_zip(self, other: HCons<H2,T2>) -> Self::HZipped {
// 		HCons{
// 			head:HCons { head: self.head, tail: other.head },
// 			tail:self.tail.h_zip(other.tail)
// 		}
// 	}
// }

/// zip 2 hlist by HCons
/// 
/** 
```
assert_eq!(
	hlist![1,2.0,"3"].h_zip(
	hlist![hlist!["4"],hlist![5],hlist![6.0]]
	)
	,
	hlist![
		hlist![1,"4"],
		hlist![2.0,5],
		hlist!["3",6.0]
	]
);
```
*/
pub fn h_zip<A,B>(a:A,b:B)->HMapP<HZip<A,B>,MapHCons>
where A:HZippable<B,Zipped : HMappable<Poly< MapHCons >>>
{
	a.zip(b).map(Poly(MapHCons))
}

impl<A,B> HHZippable<B> for A
where A:HZippable<B,Zipped : HMappable<Poly< MapHCons >>>
{
	type HZipped=HMapP<HZip<A,B>,MapHCons>;

	fn h_zip(self, other: B) -> Self::HZipped {
		h_zip(self, other)
	}
}

// may break type infer so no
// impl<H1,T1> HHZippable<HNil> for HCons<H1,T1>
// 	where T1:HHZippable<HNil>
// {
// 	type HZipped=HCons<
// 		HCons<H1,HNil>,
// 		T1::HZipped
// 	>;

// 	fn h_zip(self, _: HNil) -> Self::HZipped {
// 		HCons{
// 			head:HCons { head: self.head, tail: HNil },
// 			tail:self.tail.h_zip(HNil)
// 		}
// 	}
// }

/// `<A as HHZippable<B>>::Zipped`
/// 
/// [`HHZippable`]
pub type HHZip<A,B>=HMapP<HZip<A,B>,MapHCons>;//<A as HHZippable<B>>::HZipped;

/// transpose a hlist like a matrix
pub trait HTranspose {

	/// output type of [`HTranspose`]
	type Transposed;

	/// transpose a hlist like a matrix
	/** 
	# Example
	```
	# use frunk::hlist;
	# use wacky_bag_hlist::utils::h_h_zippable::HTranspose;
	assert_eq!(
		hlist![
			hlist![1  ,2  ,3  ,4  ],
			hlist![1.0,2.0,3.0,4.0],
			hlist!["1","2","3","4"]
		].h_transpose(),
		hlist![
			hlist![1  ,1.0,"1"],
			hlist![2  ,2.0,"2"],
			hlist![3  ,3.0,"3"],
			hlist![4  ,4.0,"4"],
		]
	)
	```
	*/
	fn h_transpose(self)->Self::Transposed;
}

impl HTranspose for HNil {
	type Transposed = HNil;
	
	fn h_transpose(self)->Self::Transposed {
		HNil
	}
}


impl<H,T1,T2,TT,O> HTranspose for HCons<H,HCons<T1,T2>>
	where HCons<T1,T2>:HTranspose<Transposed = TT>,
	H:HHZippable<TT,HZipped = O>
{
	type Transposed = O;
	fn h_transpose(self)->Self::Transposed {
		self.head.h_zip(self.tail.h_transpose())
	}
}


impl<H,O> HTranspose for HCons<H,HNil>
where 
	H:HMappable<Poly<MapToHList>,Output = O>
{
	type Transposed = O;
	fn h_transpose(self)->Self::Transposed {
		self.head.map(Poly(MapToHList))
	}
}

type HTranspose2Func=HChainFunc<
	HList!(
		FTuple2Swap,
		FHZip,
		FHMap<Poly<MapHCons>>
	)
>;

/// A ... interesting implementation of h_transpose
/// 
/// but it needs [`HRepeat<Init,HNil>`]
pub fn h_transpose_2<A,Init>(a:A)
-> <A as HFoldRightable<Poly<HTranspose2Func>,HRepeat<Init,HNil>>>::Output
where A:HFoldRightable<Poly<HTranspose2Func>,HRepeat<Init,HNil>>,
Init: HRepeatFrom<HNil>
{
	a.foldr(Poly(
		HTranspose2Func::default()
	), h_repeat(HNil))
}

// impl<H,T,TT,O> HTranspose for HCons<H,T>
// 	where T:HTranspose<Transposed = TT>,
// 	H:HHZippable<TT,HZipped = O>
// {
// 	type Transposed = O;
// 	fn h_transpose(self)->Self::Transposed {
// 		self.head.h_zip(self.tail.h_transpose())
// 	}
// }

#[cfg(test)]
mod test{
    use frunk::{HList, hlist};
	use super::*;
	#[test]
	fn test_hhzip() {
		assert_eq!(
			hlist![1,2.0,"3","7"].h_zip(
			hlist![hlist!["4"],hlist![5],hlist![6.0],hlist![7]]
			)
			,
			hlist![
				hlist![1,"4"],
				hlist![2.0,5],
				hlist!["3",6.0],
				hlist!["7",7]
			]
		);
	}
	#[test]
	fn test_transpose(){
		assert_eq!(
			hlist![
				hlist![1,2,3,4],
				hlist![1.0,2.0,3.0,4.0],
				hlist!["1","2","3","4"]
			].h_transpose(),
			hlist![
				hlist![1,1.0,"1"],
				hlist![2,2.0,"2"],
				hlist![3,3.0,"3"],
				hlist![4,4.0,"4"],
			]
		)
	}
	
	#[test]
	fn test_transpose_fn(){
		// let a1=h_transpose_2::<_,_>(hlist![
		// 	hlist![1,2,3]
		// ]);


		// // let a=
		// // h_transpose_2(hlist![
		// // 		hlist![1,2,3,4],
		// // 		hlist![1.0,2.0,3.0,4.0],
		// // 		hlist!["1","2","3","4"]
		// // 	]);
		assert_eq!(
			h_transpose_2(hlist![
				hlist![1,2,3,4],
				hlist![1.0,2.0,3.0,4.0],
				hlist!["1","2","3","4"]
			]),
			hlist![
				hlist![1,1.0,"1"],
				hlist![2,2.0,"2"],
				hlist![3,3.0,"3"],
				hlist![4,4.0,"4"],
			]
		)
	}
}