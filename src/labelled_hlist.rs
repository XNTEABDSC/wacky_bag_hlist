/// Returns a type signature for an HList of the provided types
///
/// This is a type macro (introduced in Rust 1.13) that makes it easier
/// to write nested type signatures.
///
/// # Examples
///
/// ```
/// # use frunk_core::{hlist, HList};
/// # fn main() {
/// let h: HList!(f32, &str, Option<i32>) = hlist![13.5f32, "hello", Some(41)];
///
/// // Use "...Tail" to append another HList type at the end.
/// let h: HList!(f32, ...HList!(&str, Option<i32>)) = hlist![13.5f32, "hello", Some(41)];
/// # }
/// ```
#[macro_export]
macro_rules! LabelHList {
    () => { frunk::hlist::HNil };
    (...$Rest:ty) => { $Rest };
    ( $I:ident:$A:ty ) => { frunk::HList![ frunk::labelled::field_with_name::<$I,$A> ,] };
    ( $I:ident:$A:ty, $($tok:tt)*) => {
        frunk::hlist::HCons<$A, $crate::LabelHList![$($tok)*]>
    };
}
#[cfg(test)]
mod test{
    fn test(){
        let a1=frunk::field![z,1];
        // let a:LabelHList!(
        //     a:i32,b:f32,c:f64
        // );
    }
}