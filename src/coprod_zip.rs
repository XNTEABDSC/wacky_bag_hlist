use frunk::{Coproduct, coproduct::CNil};

/// H->(H,B)
/// 
/// only one B is needed as there is only one H
pub trait CoprodZip<B>{
    /// Output of [CoprodZip<B>]
    type Output;
    
    /// H->(H,B)
    /// 
    /// only one B is needed as there is only one H
    fn zip(self,b:B)->Self::Output;
}

// impl<B> CoprodZip<B> for CNil {
//     type Output=CNil;

//     fn zip(self,_b:B)->Self::Output {
//         panic!()
//     }
// }

impl<B,H> CoprodZip<B> for Coproduct<H,CNil>
{
    type Output=Coproduct<(H,B),CNil>;

    fn zip(self,b:B)->Self::Output {
        match self {
            Coproduct::Inl(h) => Coproduct::Inl((h,b)),
            Coproduct::Inr(n) => Coproduct::Inr(n),
        }
    }
}

impl<B,H,H2,T> CoprodZip<B> for Coproduct<H,Coproduct<H2,T>>
where Coproduct<H2,T> :CoprodZip<B>
{
    type Output=Coproduct<(H,B), <Coproduct<H2,T> as CoprodZip<B>>::Output >;

    fn zip(self,b:B)->Self::Output {
        match self {
            Coproduct::Inl(h) => Coproduct::Inl((h,b)),
            Coproduct::Inr(t1) => {
                Coproduct::Inr(
                    t1.zip(b)
                )
            },
        }
    }
}

// impl<B,H,H2,T> CoprodZip<B> for Coproduct<H,Coproduct<H2,T>>
// where T:CoprodZip<B>
// {
//     type Output=Coproduct<(H,B),Coproduct<(H2,B),T::Output>>;

//     fn zip(self,b:B)->Self::Output {
//         match self {
//             Coproduct::Inl(h) => Coproduct::Inl((h,b)),
//             Coproduct::Inr(t1) => {
//                 Coproduct::Inr(
//                     match t1 {
//                         Coproduct::Inl(h2) => Coproduct::Inl((h2,b)),
//                         Coproduct::Inr(t2) => Coproduct::Inr(t2.zip(b)),
//                     }
//                 )
//             },
//         }
//     }
// }