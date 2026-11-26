use frunk::{Coproduct, coproduct::CNil};

/// Append 2 Coprod
pub trait CoprodAppend<B> {
    /// type result of 2 Coprod
    type Output;
}

impl<B> CoprodAppend<B> for CNil {
    type Output=B;
}

impl<H,T,B> CoprodAppend<B> for Coproduct<H,T>
where T:CoprodAppend<B>
{
    type Output = Coproduct<H,T::Output>;
}