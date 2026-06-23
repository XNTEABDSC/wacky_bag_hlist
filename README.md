# Some tool for frunk::hlist

## `Func`(in frunk) , `BijectiveFunc` , `TypeFunc` and `BijectiveTypeFunc`

and `ChainFunc`, 

## macros to implement them

```rust
let func=impl_func_clause!(<T>{where T:Add<T>+Clone}: (T) -> (<T as Add<T>>::Output) |i|i.clone()+i);
assert_eq!(
  hlist![1,2.0,3usize].map(Poly(func)),
  hlist![2,4.0,6usize]
);
```

## `HTranspose`

```rust
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

## `HMappableFrom`

```rust
struct NTS<T>(pub T);
new_new_type_func!(NTS MapS);
struct NTC<T>(pub T);
new_new_type_func!(NTC MapC);
struct A(pub i32);
#[derive(Debug,PartialEq, Eq)]
struct B(pub i32);
#[derive(Debug,PartialEq, Eq)]
struct C(pub i32);
fn sum_a_c(hlist_pat![a,c]:HList!(A,C))->i32{
  a.0+c.0
}

#[test]
fn test(){
  let l=hlist!(NTS(A(1)),NTC(A(2)),NTS(B(3)),NTC(B(4)),NTS(C(5)),NTC(C(6)));
  let (a,b)=l.sculpt();
  assert_eq!(
    sum_a_c(
      HMappableFrom::output_map(
        a, 
        Poly(ReverseFunc(MapC)))),
    2+6);
  // let c:HList!(B,C)=b.sculpt().0.map(Poly(ReverseFunc(MapS))); // error: type annotations needed for `sculpt`
  let c:HList!(B,C)=HMappableFrom::output_map(
      b.sculpt().0, 
      Poly(ReverseFunc(MapS)));
  assert_eq!(c,hlist![B(3),C(5)]);
}
```

## `HRepeat`

```rust
assert_eq!(
	hlist![1,2.0,"3"].h_zip(
		// <HRepeat<_,HNil> as Default>::default()
		h_repeat(HNil)
	),
	hlist![HCons{head:1,tail:HNil},HCons{head:2.0,tail:HNil},hlist!("3")]
)
```
