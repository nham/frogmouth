pub use std::collections::hashmap::{HashMap, MoveEntries};
use std::fmt::Show;

use parsers::{SymParser, AltParser, ConcatParser};

mod parsers;

trait Parser<S, T, I: Iterator<(T, S)>> {
    fn parse<'a>(&self, state: S) -> I;
}


fn print_entries<'a, A: Show, B: Show>(mut entries: MoveEntries<A, B>) {
    print!("[");
    for e in entries {
        print!("{} ", e);
    }
    println!("]");
}



fn main() {
    let ap = SymParser::new('a');
    let bp = SymParser::new('b');
    let cp = SymParser::new('c');

    let stream1 = vec!('a', 'b', 'c', 'd');
    let stream2 = vec!('b', 'b', 'c', 'd');

    let stream3 = vec!('a', 'c', 'd');
    let stream4 = vec!('b', 'c', 'd');

    let res1 = ap.parse(stream1.as_slice());
    print!("testing a: ");
    print_entries(res1);

    let res2 = bp.parse(stream2.as_slice());
    print!("testing b: ");
    print_entries(res2);

    let res3 = ap.parse(stream2.as_slice());
    print!("testing a again: ");
    print_entries(res3);

    /*
    let alt_ab = AltParser::new(ap, bp);

    let res4 = alt_ab.parse(stream1.as_slice());
    let res5 = alt_ab.parse(stream2.as_slice());
    println!("testing alt: {}\n{}", res4, res5);

    let concat_alt_ab_c = ConcatParser::new(alt_ab, cp);

    let res6 = concat_alt_ab_c.parse(stream3.as_slice());
    let res7 = concat_alt_ab_c.parse(stream4.as_slice());
    println!("testing concat: {}\n{}", res6, res7);
    */

}
