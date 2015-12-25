#[macro_use]
extern crate nom;

use std::iter::Iterator;

struct ChunkIterator {
    first_chunk: Box<[u8]>,
    second_chunk: Box<[u8]>,
    index: usize
}

impl Iterator for ChunkIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        let first_len = self.first_chunk.len();

        if self.index < first_len {
            result = Some(self.first_chunk[self.index]);
            self.index += 1;
        } else if (self.index-first_len) < self.second_chunk.len() {
            result = Some(self.second_chunk[self.index-first_len]);
            self.index += 1;
        }
        result
    }
}

#[test]
fn it_works() {
    let chunk_a = Box::new([1, 2, 3]);
    let chunk_b = Box::new([4, 5, 6]);
    let iter = ChunkIterator{
        first_chunk: chunk_a,
        second_chunk: chunk_b,
        index: 0
    };

    for (i,n) in iter.enumerate() {
        assert_eq!(n as usize, i+1);
    }
}
