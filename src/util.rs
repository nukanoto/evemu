use num_bigint::ToBigUint;

use crate::Uint256;

pub trait Uint256Util
where
    Self: ToBigUint + Sized,
{
    // fit bigint to uint256
    fn fit(&self) -> Self;
}

impl Uint256Util for Uint256 {
    fn fit(&self) -> Self {
        let a = self.to_biguint().unwrap().to_u32_digits();
        let b = if a.len() >= 8 {
            a[a.len() - 8..].to_vec()
        } else {
            a
        };
        Self::new(b)
    }
}
