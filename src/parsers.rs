use super::{Parser, ParseResult, ParseTree, Leaf, Concat};


pub struct SymParser {
    sym: char,
}

impl SymParser {
    pub fn new(c: char) -> SymParser {
        SymParser { sym: c }
    }
}

impl Parser<ParseTree<char>, char> for SymParser {
    fn parse<'a>(&self, state: &'a [char]) -> ParseResult<'a, ParseTree<char>, char> {
        match state.get(0) {
            None => vec!(),
            Some(sym) => {
                if *sym == self.sym {
                    vec!( (Leaf(self.sym), state.tailn(1)) )
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

impl<S,T, P: Parser<T,S>, Q: Parser<T,S>> Parser<T,S> for AltParser<P, Q> {
    fn parse<'a>(&self, state: &'a [S]) -> ParseResult<'a, T, S> {
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

impl<S, T: Clone, P: Parser<ParseTree<T>, S>, Q: Parser<ParseTree<T>, S>> Parser<ParseTree<T>, S> for ConcatParser<P, Q> {
    fn parse<'a>(&self, state: &'a [S]) -> ParseResult<'a, ParseTree<T>, S> {
        let mut p1_parse = self.p1.parse(state);

        let mut out = vec!();
        for (tree, rem) in p1_parse.move_iter() {
            for (tree2, rem2) in self.p2.parse(rem).move_iter() {
                out.push( (Concat(box tree.clone(), box tree2), rem2) );
            }
        }
        out
    }
}

