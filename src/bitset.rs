use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BitSet(pub u32);

impl BitSet {
    pub fn zero() -> BitSet {
        BitSet(0)
    }
    pub fn from_3bits(b0: usize, b1: usize, b2: usize) -> BitSet {
        BitSet(1 << b0 | 1 << b1 | 1 << b2)
    }
    pub fn from_4bits(b0: usize, b1: usize, b2: usize, b3: usize) -> BitSet {
        BitSet(1 << b0 | 1 << b1 | 1 << b2 | 1 << b3)
    }
    pub fn set(&mut self, index: usize) {
        self.0 |= 1 << index;
    }
    pub fn merge(self, other: BitSet) -> BitSet {
        BitSet(self.0 | other.0)
    }
    pub fn intersect(self, other: BitSet) -> BitSet {
        BitSet(self.0 & other.0)
    }
    pub fn get(self, index: usize) -> bool {
        (self.0 & (1 << index)) != 0
    }
    pub fn empty(self) -> bool {
        self.0 == 0
    }
    #[cfg(test)]
    pub fn from_u32(data: u32) -> BitSet {
        BitSet(data)
    }
    #[cfg(test)]
    pub fn invert(self) -> BitSet {
        BitSet(!self.0)
    }
    #[cfg(test)]
    pub fn count(self) -> usize {
        self.0.count_ones() as usize
    }
    #[cfg(test)]
    pub fn lowest(self) -> Option<usize> {
        if self.0 == 0 { None } else { Some(self.0.trailing_zeros() as usize) }
    }
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

impl fmt::Display for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BitSet[")?;
        let mut index = 1;
        let mut val = self.0;
        while val != 0 {
            if (val & index) != 0 {
                write!(f, "1, ")?;
                val ^= index;
            } else {
                write!(f, "0, ")?;
            }
            index <<= 1;
        }
        write!(f, "zeros]")
    }
}

impl Iterator for BitSet {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.0 == 0 {
            None
        } else {
            let bit = self.0.trailing_zeros() as usize;
            self.0 &= self.0 - 1; // clear lowest set bit
            Some(bit)
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn merge() {
        assert_eq!(
            super::BitSet::zero().merge(super::BitSet::zero()),
            super::BitSet::zero()
        );
        assert_eq!(
            super::BitSet(0b01).merge(super::BitSet(0b00)),
            super::BitSet(0b01)
        );
        assert_eq!(
            super::BitSet(0b00).merge(super::BitSet(0b10)),
            super::BitSet(0b10)
        );
        assert_eq!(
            super::BitSet(0b01).merge(super::BitSet(0b10)),
            super::BitSet(0b11)
        );
        assert_eq!(
            super::BitSet(0b11).merge(super::BitSet(0b11)),
            super::BitSet(0b11)
        );
    }

    #[test]
    fn intersect() {
        assert_eq!(
            super::BitSet::zero().intersect(super::BitSet::zero()),
            super::BitSet::zero()
        );
        assert_eq!(
            super::BitSet(0b01).intersect(super::BitSet(0b00)),
            super::BitSet(0b00)
        );
        assert_eq!(
            super::BitSet(0b00).intersect(super::BitSet(0b10)),
            super::BitSet(0b00)
        );
        assert_eq!(
            super::BitSet(0b01).intersect(super::BitSet(0b10)),
            super::BitSet(0b00)
        );
        assert_eq!(
            super::BitSet(0b11).intersect(super::BitSet(0b11)),
            super::BitSet(0b11)
        );
    }

    #[test]
    fn invert() {
        assert_eq!(
            super::BitSet(0b0000_0000_0000_0000_0000_0000_0000_0000).invert(),
            super::BitSet(0b1111_1111_1111_1111_1111_1111_1111_1111)
        );
        assert_eq!(
            super::BitSet(0b1111_1111_1111_1111_1111_1111_1111_1111).invert(),
            super::BitSet(0b0000_0000_0000_0000_0000_0000_0000_0000)
        );
        assert_eq!(
            super::BitSet(0b1111_1111_1111_1111_0000_0000_0000_0000).invert(),
            super::BitSet(0b0000_0000_0000_0000_1111_1111_1111_1111)
        );
    }

    #[test]
    fn count() {
        assert_eq!(
            super::BitSet(0b0000_0000_0000_0000_0000_0000_0000_0000).count(),
            0
        );
        assert_eq!(
            super::BitSet(0b1111_1111_1111_1111_1111_1111_1111_1111).count(),
            32
        );
        assert_eq!(
            super::BitSet(0b1111_1111_1111_1111_0000_0000_0000_0000).count(),
            16
        );
        assert_eq!(
            super::BitSet(0b0000_0000_0000_0000_1111_1111_1111_1111).count(),
            16
        );
    }

    #[test]
    fn lowest() {
        assert_eq!(
            super::BitSet(0b0000_0000_0000_0000_0000_0000_0000_0000).lowest(),
            None
        );
        assert_eq!(
            super::BitSet(0b1111_1111_1111_1111_1111_1111_1111_1111).lowest(),
            Some(0)
        );
        assert_eq!(
            super::BitSet(0b1111_1111_1111_1111_0000_0000_0000_0000).lowest(),
            Some(16)
        );
    }

    #[test]
    fn empty() {
        assert_eq!(super::BitSet(0b0000_0000).empty(), true);
        assert_eq!(super::BitSet(0b1000_0000).empty(), false);
        assert_eq!(super::BitSet(0b0100_1100).empty(), false);
        assert_eq!(super::BitSet(0b1100_1101).empty(), false);
        assert_eq!(super::BitSet(0b1111_1111).empty(), false);
    }

    #[test]
    fn iterate() {
        let mut b = super::BitSet(0b0100_1010);
        assert_eq!(b.next(), Some(1));
        assert_eq!(b.next(), Some(3));
        assert_eq!(b.next(), Some(6));
        assert_eq!(b.next(), None);
    }

    #[test]
    fn iterate_empty() {
        let mut b = super::BitSet(0);
        assert_eq!(b.next(), None);
    }

    #[test]
    fn iterate_bit0() {
        let mut b = super::BitSet(1);
        assert_eq!(b.next(), Some(0));
        assert_eq!(b.next(), None);
    }

    #[test]
    fn iterate_bit31() {
        let mut b = super::BitSet(1 << 31);
        assert_eq!(b.next(), Some(31));
        assert_eq!(b.next(), None);
    }

    #[test]
    fn iterate_all_bits() {
        let bits: Vec<usize> = super::BitSet(u32::MAX).collect();
        assert_eq!(bits, (0..32).collect::<Vec<_>>());
    }

    #[test]
    fn count_single_bit() {
        assert_eq!(super::BitSet(1).count(), 1);
        assert_eq!(super::BitSet(1 << 31).count(), 1);
    }

    #[test]
    fn lowest_single_bits() {
        assert_eq!(super::BitSet(1).lowest(), Some(0));
        assert_eq!(super::BitSet(1 << 15).lowest(), Some(15));
        assert_eq!(super::BitSet(1 << 31).lowest(), Some(31));
    }
}
