# Some tool for frunk::hlist

## Extending `Func`(in frunk) by `BijectiveFunc` , `TypeFunc` and `BijectiveTypeFunc`

and utils `ChainFunc`, `InverseFunc`

## macros to implement them

```rust
new_struct_func!(
    pub AddSelf // name, can have <T>
    impl<T>{where T:Add<T>+Clone} // impl with generic parameter, can add where clause inside a {}
    :
    (T) -> (<T as Add<T>>::Output) // input output type, -> is one dir (injective), <-> is bijective
    |i|i.clone()+i
);
assert_eq!(
  hlist![1,2.0,3usize].map(Poly(AddSelf)),
  hlist![2,4.0,6usize]
);
```

or

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
// some types --------
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct NTS<T>(pub T);
new_new_type_func!(NTS MapS);
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct NTC<T>(pub T);
new_new_type_func!(NTC MapC);
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct A(pub i32);
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct B(pub i32);
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct C(pub i32);

/// adding A and C
fn sum_a_c(hlist_pat![a,c]:HList!(A,C))->i32{
  a.0+c.0
}

#[test]
fn test(){
  let l=hlist!(NTS(A(1)),NTC(A(2)),NTS(B(3)),NTC(B(4)),NTS(C(5)),NTC(C(6)));

  // assert_eq!(
  //  sum_a_c(
  //    l.clone().sculpt().0.map(Poly(InverseFunc(MapC)))), // type annotations needed. output of sculpt is unclear so map is impossible.
  //  2+6);
  
  assert_eq!(
    sum_a_c(
      HMappableFrom::output_map(
        l.clone().sculpt().0, // type inferred by parameter of `sum_a_c` , which is `HList!(NTC<A>, NTC<C>)`
        Poly(InverseFunc(MapC)))),
    2+6);

  // can sculpt twice
  // `a:HList!(NTC<A>,NTC<C>)`, `b:HList!(NTS<A>, NTS<B>, NTC<B>, NTS<C>)`
  let (a,b)=l.sculpt();

  assert_eq!(
    sum_a_c(
      HMappableFrom::output_map(
        a, 
        Poly(InverseFunc(MapC)))),
    2+6);
  
  // let c:HList!(B,C)=b.sculpt().0.map(Poly(InverseFunc(MapS))); // error: type annotations needed for `sculpt`
  
  let c:HList!(B,C)=HMappableFrom::output_map(
      b.sculpt().0, 
      Poly(InverseFunc(MapS)));
  
  assert_eq!(c,hlist![B(3),C(5)]);
  
}
```

## `HRepeat`

```rust
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

## others

we assume all `impl Func` also `impl Default` so we use variable if possible, and use `F::default()` if not.