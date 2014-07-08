use super::{Parser, ParseResult, ParseTree, Leaf, Concat};


pub struct SymParser {
    sym: char,
}

impl SymParser {
    pub fn new(c: char) -> SymParser {
        SymParser { sym: c }
    }
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


pub struct AltParser<P, Q> {
    p1: P,
    p2: Q,
}

impl<P, Q> AltParser<P, Q> {
    pub fn new(p1: P, p2: Q) -> AltParser<P, Q> {
        AltParser { p1: p1, p2: p2 }
    }
}

impl<S,T, P: Parser<S,T>, Q: Parser<S,T>> Parser<S,T> for AltParser<P, Q> {
    fn parse<'a>(&self, state: &'a [S]) -> ParseResult<'a, S, T> {
        let mut p1_parse = self.p1.parse(state);
        p1_parse.push_all_move( self.p2.parse(state) );
        p1_parse
    }
}


pub struct ConcatParser<P, Q> {
    p1: P,
    p2: Q,
}

impl<P, Q> ConcatParser<P, Q> {
    pub fn new(p1: P, p2: Q) -> ConcatParser<P, Q> {
        ConcatParser { p1: p1, p2: p2 }
    }
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
