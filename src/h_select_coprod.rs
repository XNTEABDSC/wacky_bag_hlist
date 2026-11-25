use std::marker::PhantomData;

use frunk::{Func, Poly, coproduct::CoprodInjector, hlist::HFoldLeftable};

use crate::{h_list_helpers::FIdent, h_with_index::HWithIndex, hlist_coprod::HListToCoprod};

/// fold fn for [`h_select_coprod`]
#[derive(Debug,Default,Clone, Copy)]
pub struct FoldSelectCoprod<SelectorFn>(pub SelectorFn);
/// state/result of [`h_select_coprod`]
#[derive(Debug,Clone, Copy)]
pub enum FoldSelectCoprodState<SelectorState,ResultCoprod>
{
    /// still finding, containing the state
    Finding(
        SelectorState
    ),
    /// containing the result
    Found(
        ResultCoprod
    )
}

impl<T,TO,Index,ResultCoprod,SelectorFn,SelectorState> Func<(FoldSelectCoprodState<SelectorState,ResultCoprod>,(T,PhantomData<Index>))> for FoldSelectCoprod<SelectorFn>
where SelectorFn:Func<(SelectorState,T) , Output = Result<TO,SelectorState>>,
    ResultCoprod:CoprodInjector<TO,Index>
{
    type Output=FoldSelectCoprodState<SelectorState,ResultCoprod>;

    fn call(i: (FoldSelectCoprodState<SelectorState,ResultCoprod>,(T,PhantomData<Index>))) -> Self::Output {
        match i.0 {
            FoldSelectCoprodState::Finding(state) => {
                let res=SelectorFn::call((state,i.1.0));
                match res {
                    Ok(to) => FoldSelectCoprodState::Found(ResultCoprod::inject(to)),
                    Err(state) => FoldSelectCoprodState::Finding(state),
                }
            },
            FoldSelectCoprodState::Found(res) => FoldSelectCoprodState::Found(res),
        }
    }
}

/// helper trait for [`h_select_coprod`]
/// 
/// this could help you to get rid of where spam
pub trait HSelectableCoprod<SelectFn,SelectorState>
where Self:
    HListToCoprod+
    HWithIndex<FIdent, 
    Output : HFoldLeftable<
        Poly<FoldSelectCoprod<SelectFn>>,
        FoldSelectCoprodState<SelectorState,<Self as HListToCoprod>::Coprod>,
        Output = FoldSelectCoprodState<SelectorState, <Self as HListToCoprod>::Coprod >
    >
>+Sized
{
    /// the Coprod of value
    type Coprod;
    /// ther return type of [`Self::select_coprod`]
    type Result;
    /// [`h_select_coprod`]
    fn select_coprod(self,select_fn:SelectFn,select_state:SelectorState)->Self::Result;
}

impl<H,SelectFn,SelectorState> HSelectableCoprod<SelectFn,SelectorState> for H
where H:
    HListToCoprod+
    HWithIndex<FIdent, 
    Output : HFoldLeftable<
        Poly<FoldSelectCoprod<SelectFn>>,
        FoldSelectCoprodState<SelectorState,<H as HListToCoprod>::Coprod>,
        Output = FoldSelectCoprodState<SelectorState, <H as HListToCoprod>::Coprod >
    >
>
{
    type Coprod=<Self as HListToCoprod>::Coprod;
    type Result=FoldSelectCoprodState<SelectorState, <H as HListToCoprod>::Coprod >;
    fn select_coprod(self,select_fn:SelectFn,select_state:SelectorState)->Self::Result{
        h_select_coprod(self, select_fn, select_state)
    }
}

/// folding `values`, test with SelectFn(select_state,value)
/// 
/// if returning `Ok`, return Coprod of that value
/// 
/// else keep folding
pub fn h_select_coprod<H,SelectFn,SelectorState>(values:H,select_fn:SelectFn,select_state:SelectorState)
->FoldSelectCoprodState<SelectorState, <H as HListToCoprod>::Coprod >
where H:
    HListToCoprod+
    HWithIndex<FIdent, 
    Output : HFoldLeftable<
        Poly<FoldSelectCoprod<SelectFn>>,
        FoldSelectCoprodState<SelectorState,<H as HListToCoprod>::Coprod>,
        Output = FoldSelectCoprodState<SelectorState, <H as HListToCoprod>::Coprod >
    >
>
{
    let values_index=values.with_index();
    values_index.foldl(Poly(FoldSelectCoprod(select_fn)), FoldSelectCoprodState::Finding(select_state))
}

#[cfg(test)]
mod test{
    use frunk::hlist;
    use super::*;
    use crate::new_struct_func;


    new_struct_func!(TestSelect
        impl<T>{where T:Into<f64>+Copy}:
        ((f64,T))->(Result<T,f64>)
        | (n,v) |{
            if v.into()>n {
                Ok(v)
            }else {
                Err(n-1.0)
            }
        }
    );
    #[test]
    fn test1(){
        let values=hlist!(1i32,3f64,4f32);
        let falling_num=3.5;
        // let values_index=HWithIndex::<FIdent>::with_index(values);

        // type B=<A as HListToCoprod>::Coprod;


        // let init=FoldSelectCoprodState::<_,B>::Finding(falling_num);

        // let res=values_index.foldl(Poly(a_select), init);
        let res=h_select_coprod(values, TestSelect, falling_num);
        println!("{:?}",res);
    }
}