use tree::{Tree, Node};

mod tree;

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
        let p1_parse = self.p1.parse(state);
        if p1_parse.len() > 0 {
            p1_parse
        } else {
            self.p2.parse(state)
        }
    }
}


struct SymParser {
    sym: char,
}

impl Parser<char, Tree<char>> for SymParser {
    fn parse<'a>(&self, state: &'a [char]) -> ParseResult<'a, char, Tree<char>> {
        match state.get(0) {
            None => vec!(),
            Some(sym) => {
                if *sym == self.sym {
                    vec!((state.tailn(1), Node(self.sym, vec!())))
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
