#[derive(Debug, Clone, Copy)]
pub struct BitSet(u64);

impl BitSet {
    pub fn new() -> BitSet {
        BitSet(0)
    }

    pub fn add(&mut self, value: usize) {
        self.0 |= Self::flag(value)
    }

    pub fn remove(&mut self, value: usize) {
        self.0 &= !Self::flag(value)
    }

    pub fn contains(&self, value: usize) -> bool {
        self.0 & Self::flag(value) != 0
    }

    pub fn for_each<F: FnMut(usize)>(&self, mut f: F) {
        let mut value = self.0;
        let mut index = 0;
        while value != 0 {
            let trailing = value.trailing_zeros() as usize;
            index += trailing;
            f(index);
            value >>= trailing + 1;
            index += 1;
        }
    }

    fn flag(value: usize) -> u64 {
        1 << value
    }
}
