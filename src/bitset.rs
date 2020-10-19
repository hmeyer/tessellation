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
        let mut result = 0;
        for p in 0..32 {
            if (self.0 & (1 << p)) != 0 {
                result += 1;
            }
        }
        result
    }
    #[cfg(test)]
    pub fn lowest(self) -> Option<usize> {
        for p in 0..32 {
            if (self.0 & (1 << p)) != 0 {
                return Some(p);
            }
        }
        None
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
            let old = self.0;
            self.0 &= self.0 - 1;
            Some(match (!self.0) & old {
                0x0000_0001 => 0,
                0x0000_0002 => 1,
                0x0000_0004 => 2,
                0x0000_0008 => 3,
                0x0000_0010 => 4,
                0x0000_0020 => 5,
                0x0000_0040 => 6,
                0x0000_0080 => 7,
                0x0000_0100 => 8,
                0x0000_0200 => 9,
                0x0000_0400 => 10,
                0x0000_0800 => 11,
                0x0000_1000 => 12,
                0x0000_2000 => 13,
                0x0000_4000 => 14,
                0x0000_8000 => 15,
                0x0001_0000 => 16,
                0x0002_0000 => 17,
                0x0004_0000 => 18,
                0x0008_0000 => 19,
                0x0010_0000 => 20,
                0x0020_0000 => 21,
                0x0040_0000 => 22,
                0x0080_0000 => 23,
                0x0100_0000 => 24,
                0x0200_0000 => 25,
                0x0400_0000 => 26,
                0x0800_0000 => 27,
                0x1000_0000 => 28,
                0x2000_0000 => 29,
                0x4000_0000 => 30,
                0x8000_0000 => 31,
                x => panic!("not a single bit: {:?}", x),
            })
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
}
