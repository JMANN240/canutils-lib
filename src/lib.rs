use std::collections::VecDeque;

use bitvec::prelude::*;

pub mod can;
pub mod j1939;

struct MaxSizeQueue<T> {
    elements: VecDeque<T>,
    max_size: usize,
}

impl<T> MaxSizeQueue<T> {
    pub fn new(max_size: usize) -> Self {
        Self { elements: VecDeque::new(), max_size }
    }

    pub fn elements(&self) -> &VecDeque<T> {
        &self.elements
    }

    pub fn max_size(&self) -> usize {
        self.max_size
    }

    pub fn is_full(&self) -> bool {
        self.elements().len() == self.max_size()
    }

    pub fn add(&mut self, element: T) -> Option<T> {
        self.elements.push_back(element);

        (self.elements().len() > self.max_size()).then(|| self.elements.pop_front()).flatten()
    }
}

pub fn stuff<T: BitStore, B: AsRef<BitSlice<T, Msb0>>>(unstuffed_bits: B, n: usize) -> BitVec<T, Msb0> {
    let unstuffed_bits = unstuffed_bits.as_ref();

    let mut stuffed_bits = BitVec::new();
    let mut window = MaxSizeQueue::new(n);

    for (index, bit) in unstuffed_bits.iter().enumerate() {
        stuffed_bits.push(*bit);
        window.add(*bit);

        if window.is_full() && index < unstuffed_bits.len().saturating_sub(13) {
            if !window.elements().contains(&true) {
                stuffed_bits.push(true);
                window.add(true);
            } else if !window.elements().contains(&false) {
                stuffed_bits.push(false);
                window.add(false);
            }
        }
    }

    stuffed_bits
}

pub fn unstuff<T: BitStore, B: AsRef<BitSlice<T, Msb0>>>(stuffed_bits: B, n: usize) -> BitVec<T, Msb0> {
    let stuffed_bits = stuffed_bits.as_ref();

    let mut unstuffed_bits = BitVec::new();
    let mut window = MaxSizeQueue::new(n);

    for (index, bit) in stuffed_bits.iter().enumerate() {
        if index >= stuffed_bits.len().saturating_sub(13) || !window.is_full() || (window.elements().contains(&true) && window.elements().contains(&false)) {
            unstuffed_bits.push(*bit);
        }

        window.add(*bit);
    }

    unstuffed_bits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_size_queue() {
        let mut queue = MaxSizeQueue::new(5);
        assert_eq!(queue.elements().len(), 0);

        queue.add(0);
        assert_eq!(queue.elements().len(), 1);
        assert_eq!(queue.elements(), &[0]);

        queue.add(1);
        assert_eq!(queue.elements().len(), 2);
        assert_eq!(queue.elements(), &[0, 1]);

        queue.add(2);
        assert_eq!(queue.elements().len(), 3);
        assert_eq!(queue.elements(), &[0, 1, 2]);

        queue.add(3);
        assert_eq!(queue.elements().len(), 4);
        assert_eq!(queue.elements(), &[0, 1, 2, 3]);

        queue.add(4);
        assert_eq!(queue.elements().len(), 5);
        assert_eq!(queue.elements(), &[0, 1, 2, 3, 4]);

        queue.add(5);
        assert_eq!(queue.elements().len(), 5);
        assert_eq!(queue.elements(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bit_stuffing() {
        let bits = bitvec![usize, Msb0;
            0,
            0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0,
            0,
            0,
            0,
            0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 1,
            1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0 ,0, 1, 1,
            1,
            0,
            1,
            1, 1, 1, 1, 1, 1, 1,
            1, 1, 1,
        ];

        assert_eq!(stuff(bits, 5), bitvec![
            0,
            0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0,
            0,
            0,
            0, 1,
            0, 0, 0, 1,
            0, 0, 0, 0, 0, 1, 0, 0, 1,
            1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0 ,0, 1, 1,
            1,
            0,
            1,
            1, 1, 1, 1, 1, 1, 1,
            1, 1, 1,
        ]);
    }

    #[test]
    fn test_bit_unstuffing() {
        let bits = bitvec![usize, Msb0;
            0,
            0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0,
            0,
            0,
            0, 1,
            0, 0, 0, 1,
            0, 0, 0, 0, 0, 1, 0, 0, 1,
            1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0 ,0, 1, 1,
            1,
            0,
            1,
            1, 1, 1, 1, 1, 1, 1,
            1, 1, 1,
        ];

        assert_eq!(unstuff(bits, 5), bitvec![
            0,
            0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0,
            0,
            0,
            0,
            0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 1,
            1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0 ,0, 1, 1,
            1,
            0,
            1,
            1, 1, 1, 1, 1, 1, 1,
            1, 1, 1,
        ]);
    }
}
