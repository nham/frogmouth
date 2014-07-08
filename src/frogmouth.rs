use parsers::{SymParser, AltParser, ConcatParser};

mod parsers;


type ParseResult<'a, S, T> = Vec<(&'a [S], T)>;

trait Parser<S,T> {
    fn parse<'a>(&self, state: &'a [S]) -> ParseResult<'a, S, T>;
}

#[deriving(Show)]
enum ParseTree<T> {
    Nil,
    Leaf(T),
    Concat(Box<ParseTree<T>>, Box<ParseTree<T>>),
}

impl<T: Clone> Clone for ParseTree<T> {
    fn clone(&self) -> ParseTree<T> {
        match *self {
            Nil => Nil,
            Leaf(ref v) => Leaf(v.clone()),
            Concat(ref a, ref b) => Concat(a.clone(), b.clone()),
        }
    }
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
    println!("testing a: {}", res1);
    let res2 = bp.parse(stream2.as_slice());
    println!("testing b: {}", res2);
    let res3 = ap.parse(stream2.as_slice());
    println!("testing a again: {}", res3);

    let alt_ab = AltParser::new(ap, bp);

    let res4 = alt_ab.parse(stream1.as_slice());
    let res5 = alt_ab.parse(stream2.as_slice());
    println!("testing alt: {}\n{}", res4, res5);

    let concat_alt_ab_c = ConcatParser::new(alt_ab, cp);

    let res6 = concat_alt_ab_c.parse(stream3.as_slice());
    let res7 = concat_alt_ab_c.parse(stream4.as_slice());
    println!("testing concat: {}\n{}", res6, res7);

}