type ParseResult<'a, S, T> = Vec<(&'a [S], T)>;

trait Parser<S,T> {
    fn parse<'a>(&self, state: &'a [S]) -> ParseResult<'a, S, T>;
}

struct AltParser<P, Q> {
    p1: P,
    p2: Q,
}

impl<S,T, P: Parser<S,T>, Q: Parser<S,T>> Parser<S,T> for AltParser<P, Q> {
    fn parse<'a>(&self, state: &'a [S]) -> ParseResult<'a, S, T> {
        let mut p1_parse = self.p1.parse(state);
        p1_parse.push_all_move( self.p2.parse(state) );
        p1_parse
    }
}

#[deriving(Show)]
enum ParseTree<T> {
    Nil,
    Leaf(T),
    Alt(Box<ParseTree<T>>, Box<ParseTree<T>>),
    Concat(Box<ParseTree<T>>, Box<ParseTree<T>>),
}

impl<T: Clone> Clone for ParseTree<T> {
    fn clone(&self) -> ParseTree<T> {
        match *self {
            Nil => Nil,
            Leaf(ref v) => Leaf(v.clone()),
            Alt(ref a, ref b) => Alt(a.clone(), b.clone()),
            Concat(ref a, ref b) => Concat(a.clone(), b.clone()),
        }
    }
}

struct ConcatParser<P, Q> {
    p1: P,
    p2: Q,
}

impl<S, T: Clone, P: Parser<S,ParseTree<T>>, Q: Parser<S,ParseTree<T>>> Parser<S,ParseTree<T>> for ConcatParser<P, Q> {
    fn parse<'a>(&self, state: &'a [S]) -> ParseResult<'a, S, ParseTree<T>> {
        let mut p1_parse: Vec<(&'a [S], ParseTree<T>)> = self.p1.parse(state);

        let mut out = vec!();
        for (rem, tree) in p1_parse.move_iter() {
            for (rem2, tree2) in self.p2.parse(rem).move_iter() {
                out.push((rem2, Concat(box tree.clone(), box tree2)));
            }
        }
        out
    }
}


struct SymParser {
    sym: char,
}

impl Parser<char, ParseTree<char>> for SymParser {
    fn parse<'a>(&self, state: &'a [char]) -> ParseResult<'a, char, ParseTree<char>> {
        match state.get(0) {
            None => vec!(),
            Some(sym) => {
                if *sym == self.sym {
                    vec!((state.tailn(1), Leaf(self.sym)))
                } else {
                    vec!()
                }
            },
        }

    }
}


fn main() {
    let ap = SymParser { sym: 'a' };
    let bp = SymParser { sym: 'b' };

    let stream1 = vec!('a', 'b', 'c', 'd');
    let stream2 = vec!('b', 'b', 'c', 'd');

    let res1 = ap.parse(stream1.as_slice());
    println!("{}", res1);
    let res2 = bp.parse(stream2.as_slice());
    println!("{}", res2);
    let res3 = ap.parse(stream2.as_slice());
    println!("{}", res3);

    let alt_ab = AltParser { p1: ap, p2: bp };

    let res4 = alt_ab.parse(stream1.as_slice());
    let res5 = alt_ab.parse(stream2.as_slice());
    println!("{}\n{}", res4, res5);

}
