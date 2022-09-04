use num256::Uint256;
use num_bigint::BigUint;

pub trait UtilFromInto<T>
where
    Self: Sized,
{
    fn util_from(_: T) -> Self;
    fn util_into(self) -> T;
}

impl UtilFromInto<Uint256> for usize {
    fn util_from(n: Uint256) -> Self {
        n.to_u64_digits()[0] as usize
    }

    fn util_into(self) -> Uint256 {
        <Uint256 as UtilFromInto<usize>>::util_from(self)
    }
}

impl UtilFromInto<usize> for Uint256 {
    fn util_from(n: usize) -> Self {
        num256::Uint256(BigUint::from(n))
    }

    fn util_into(self) -> usize {
        <usize as UtilFromInto<Uint256>>::util_from(self)
    }
}
