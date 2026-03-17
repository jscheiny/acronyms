use num::PrimInt;

pub type LetterSet = BitSet<u64>;

// TODO Allow separate index type
#[derive(Debug, Clone, Copy)]
pub struct BitSet<T: PrimInt>(pub T);

impl<T> BitSet<T>
where
    T: PrimInt + std::ops::BitOrAssign<T> + std::ops::ShrAssign<usize>,
{
    pub fn new() -> BitSet<T> {
        BitSet(T::zero())
    }

    pub fn add(&mut self, value: usize) {
        self.0 |= Self::flag(value)
    }

    pub fn for_each<F: FnMut(usize)>(&self, mut f: F) {
        let mut value = self.0;
        let mut index = 0;
        while !value.is_zero() {
            let trailing = value.trailing_zeros() as usize;
            index += trailing;
            f(index);
            value >>= trailing + 1;
            index += 1;
        }
    }

    fn flag(value: usize) -> T {
        T::one() << value
    }
}
