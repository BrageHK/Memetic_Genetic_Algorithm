use rand::prelude::*;

pub struct BitArray {
    data: [u64; 8], // 512 bits
}

impl BitArray {
    fn new() -> Self {
        BitArray { data: [0; 8] }
    }

    pub fn new_random() -> Self {
        let mut b_arr = BitArray::new();
        for i in 0..500 {
            if random::<bool>() {
                b_arr.set_bit(i, true);
            }
        }
        b_arr
    }

    pub fn set_bit(&mut self, index: usize, value: bool) {
        if index >= 500 {
            panic!("Index out of bounds");
        }
        let chunk = index / 64;
        let bit = index % 64;
        if value {
            self.data[chunk] |= 1 << bit;
        } else {
            self.data[chunk] &= !(1 << bit);
        }
    }

    pub fn get_bit(&self, index: usize) -> bool {
        if index >= 500 {
            panic!("Index out of bounds");
        }
        let chunk = index / 64;
        let bit = index % 64;
        (self.data[chunk] >> bit) & 1 == 1
    }
}