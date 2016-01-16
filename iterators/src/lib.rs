use std::iter::Iterator;
use std::io::Write;
macro_rules! println_stderr(
    ($($arg:tt)*) => (
        if let Err(x) = writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            panic!("Unable to write to stderr: {}", x);
        }
    )
);

#[test]
fn it_works() {
    let mut c = Counter{index: 0};
    c.nth(0);
}

struct Counter {
    index: usize
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.index += 1;
        println_stderr!("count now at: {}", self.index);
        Some(self.index)
    }
}
