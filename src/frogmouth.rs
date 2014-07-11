pub use std::collections::hashmap::{HashMap, MoveEntries};
pub use std::vec::MoveItems;
use std::fmt::Show;

use parsers::{NilParser, AltParser, alt};

mod parsers;

/*****************/

fn print_iter<T: Show, I: Iterator<T>>(mut entries: I) {
    print!("[");
    for e in entries {
        print!("{} ", e);
    }
    print!("]");
}

/*
fn test_parse_input<'a, S: Show, I: Iterator<(Vec<S>, &'a [S])>, P: Parser<&'a [S], Vec<S>, I>>(p: P, inp: &'a [S]) {
    let res = p.parse(inp);
    print!("testing with input {} -- ", inp);
    print_iter(res);
    println!("");
}
*/


fn main() {
    /*
    let ap = SymParser::new('a');
    let bp = SymParser::new('b');
    let cp = SymParser::new('c');
    let dp = SymParser::new('d');

    let alt_ab = AltParser::new(ap, bp);
    let alt_cd = AltParser::new(cp, dp);

    let opt_b = OptionalParser::new(bp);


    let stream1 = vec!('a', 'b', 'c', 'd');
    let stream2 = vec!('b', 'b', 'c', 'd');

    println!("testing a: ");
    test_parse_input(ap, stream1.as_slice());

    println!("testing b: ");
    test_parse_input(bp, stream2.as_slice());

    println!("testing a again: ");
    test_parse_input(ap, stream2.as_slice());

    println!("testing alt_ab: ");
    test_parse_input(alt_ab, stream1.as_slice());

    println!("testing alt_ab again: ");
    test_parse_input(alt_ab, stream2.as_slice());

    println!("testing opt_b: ");
    test_parse_input(opt_b, stream1.as_slice());
    test_parse_input(opt_b, stream2.as_slice());

    println!("testing a ++ opt_b: ");
    let concat_a_opt_b = ConcatParser::new(ap, opt_b);
    test_parse_input(concat_a_opt_b, stream1.as_slice());


    let stream3 = vec!('a', 'c', 'd');
    let stream4 = vec!('b', 'c', 'd');

    let concat_alt_ab_c = ConcatParser::new(alt_ab, cp);

    println!("testing concat: ");
    test_parse_input(concat_alt_ab_c, stream3.as_slice());

    println!("testing concat again: ");
    test_parse_input(concat_alt_ab_c, stream4.as_slice());


    let concat_alt_ab_alt_cd = ConcatParser::new(alt_ab, alt_cd);

    let stream8 = vec!('a', 'c');
    let stream9 = vec!('b', 'c');
    let stream10 = vec!('a', 'd');
    let stream11 = vec!('b', 'd');

    let res8 = concat_alt_ab_alt_cd.parse(stream8.as_slice());
    let res9 = concat_alt_ab_alt_cd.parse(stream9.as_slice());
    let res10 = concat_alt_ab_alt_cd.parse(stream10.as_slice());
    let res11 = concat_alt_ab_alt_cd.parse(stream11.as_slice());

    println!("concatting two alt parsers:");
    print_iter(res8);
    println!("");
    print_iter(res9);
    println!("");
    print_iter(res10);
    println!("");
    print_iter(res11);
    println!("");


    let stream12 = vec!('a', 'b', 'c', 'd', 'e');
    let stream13 = vec!('a', 'b', 'c', 'a', 'b', 'c', 'd', 'e');

    println!("testing concat of abc: ");
    let concat_abc = ConcatParser::new(ConcatParser::new(ap, bp), cp);
    test_parse_input(concat_abc, stream12.as_slice());

    println!("testing concat of abcabc: ");
    let concat_abc = ConcatParser::new(concat_abc, concat_abc);
    test_parse_input(concat_abc, stream13.as_slice());


    println!("testing concat of ab?c: ");
    //let concat_a_opt_b_c = ConcatParser::new(ap, ConcatParser::new(bqp, cp));
    let concat_a_opt_b_c = ConcatParser::new(concat_a_opt_b, cp);
    let stream14 = vec!('a', 'c', 'd', 'e');
    test_parse_input(concat_a_opt_b_c, stream12.as_slice());
    test_parse_input(concat_a_opt_b_c, stream14.as_slice());
    */
}
