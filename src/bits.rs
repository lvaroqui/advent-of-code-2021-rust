pub struct Bits<'a> {
    buf: &'a [u8],
    pos: usize, // In bits
}

impl<'a> Bits<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Bits {
            buf: buffer,
            pos: 0,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn get(&mut self, bits: u32) -> u64 {
        let mut remaining = bits as u64;
        let mut res = 0u64;

        while remaining != 0 {
            let byte_index = self.pos / 8;
            let byte_start_pos = 8 - (self.pos as u64 % 8);
            let byte_end_pos = if remaining >= 8 || byte_start_pos < remaining {
                0
            } else {
                byte_start_pos - remaining
            };

            let mask = ((1u32 << byte_start_pos as u32) - 1) as u8;

            let byte = self.buf[byte_index];
            let val = (byte & mask) >> byte_end_pos;

            let read = byte_start_pos - byte_end_pos;
            self.pos += read as usize;
            remaining -= read;

            res |= (val as u64) << remaining;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_bytes() {
        let buf = vec![0b1101_0011, 0b1111_1111, 0b1010_0000, 0b1110_1000];

        let mut bits = Bits::new(&buf);

        assert_eq!(bits.pos(), 0);
        assert_eq!(bits.get(2), 0b11);
        assert_eq!(bits.pos(), 2);
        assert_eq!(bits.get(3), 0b010);
        assert_eq!(bits.pos(), 5);
        assert_eq!(bits.get(7), 0b011_1111);
        assert_eq!(bits.pos(), 12);
        assert_eq!(bits.get(16), 0b1111_1010_0000_1110);
        assert_eq!(bits.pos(), 28);
        assert_eq!(bits.get(4), 0b1000);
        assert_eq!(bits.pos(), 32);
    }

    #[test]
    fn test_read_number() {
        let buf = vec![0, 0, 1, 1];

        let mut bits = Bits::new(&buf);

        assert_eq!(bits.pos(), 0);
        assert_eq!(bits.get(32), 257);
        assert_eq!(bits.pos(), 32);
    }
}
