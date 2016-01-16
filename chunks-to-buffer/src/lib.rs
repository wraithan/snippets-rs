#[macro_use]
extern crate nom;

use std::iter::Iterator;

struct ChunkIterator<'a> {
    chunks: Vec<&'a[u8]>,
    index: usize
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        let mut offset = self.index;
        for chunk in self.chunks.iter() {
            let chunk_len = chunk.len();
            if offset < chunk_len {
                result = Some(chunk[offset]);
                self.index += 1;
                break;
            }
            offset -= offset;
        }
        result
    }
}

impl<'a> ChunkIterator<'a> {
    fn new() -> ChunkIterator<'a> {
        ChunkIterator{
            chunks: vec![],
            index: 0
        }
    }

    fn push(&mut self, chunk: &[u8]) {
        self.chunks.push(chunk);
    }
}

#[test]
fn bichunk_works() {
    let iter = ChunkIterator::New();
    iter.push(&[1, 2, 3]);
    iter.push(&[4, 5, 6]);
    for (i,n) in iter.enumerate() {
        assert_eq!(n as usize, i+1);
    }
}

#[test]
fn nom_with_bichunk() {
    
}
