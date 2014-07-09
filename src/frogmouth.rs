pub use std::collections::hashmap::{HashMap, MoveEntries};
use std::fmt::Show;

use parsers::{SymParser, AltParser, ConcatParser};

mod parsers;

// S is a stream of input symbols, T is some type representing parsed input
// I is an iterator that represents the possible matches
trait Parser<S, T, I: Iterator<(T, S)>> {
    fn parse(&self, state: S) -> I;
}


fn print_iter<T: Show, I: Iterator<T>>(mut entries: I) {
    print!("[");
    for e in entries {
        print!("{} ", e);
    }
    print!("]");
}



fn main() {
    let ap = SymParser::new('a');
    let bp = SymParser::new('b');
    let cp = SymParser::new('c');

    let stream1 = vec!('a', 'b', 'c', 'd');
    let stream2 = vec!('b', 'b', 'c', 'd');

    let res1 = ap.parse(stream1.as_slice());
    print!("testing a: ");
    print_iter(res1);
    println!("")

    let res2 = bp.parse(stream2.as_slice());
    print!("testing b: ");
    print_iter(res2);
    println!("")

    let res3 = ap.parse(stream2.as_slice());
    print!("testing a again: ");
    print_iter(res3);
    println!("")

    let alt_ab = AltParser::new(ap, bp);

    let res4 = alt_ab.parse(stream1.as_slice());
    let res5 = alt_ab.parse(stream2.as_slice());

    print!("testing alt: ");
    print_iter(res4);
    println!("");

    print!("testing alt again: ");
    print_iter(res5);
    println!("");


    let stream3 = vec!('a', 'c', 'd');
    let stream4 = vec!('b', 'c', 'd');

    let concat_alt_ab_c = ConcatParser::new(alt_ab, cp);

    let res6 = concat_alt_ab_c.parse(stream3.as_slice());
    let res7 = concat_alt_ab_c.parse(stream4.as_slice());

    print!("testing concat: ");
    print_iter(res6);
    println!("");

    print!("testing concat again: ");
    print_iter(res7);
    println!("");

}
