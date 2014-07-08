use super::{Parser, ParseResult};

pub struct SymParser {
    sym: char,
}

impl SymParser {
    pub fn new(c: char) -> SymParser {
        SymParser { sym: c }
    }
}

impl Parser<Vec<char>, char> for SymParser {
    fn parse<'a>(&self, state: &'a [char]) -> ParseResult<'a, Vec<char>, char> {
        match state.get(0) {
            None => vec!(),
            Some(sym) => {
                if *sym == self.sym {
                    vec!( (vec!(self.sym), state.tailn(1)) )
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

impl<S, T: Clone, P: Parser<Vec<T>, S>, Q: Parser<Vec<T>, S>> Parser<Vec<T>, S> for ConcatParser<P, Q> {
    fn parse<'a>(&self, state: &'a [S]) -> ParseResult<'a, Vec<T>, S> {
        let p1_parse = self.p1.parse(state);

        let mut out = vec!();
        for (tree, rem) in p1_parse.move_iter() {
            for (tree2, rem2) in self.p2.parse(rem).move_iter() {
                let mut newtree = tree.clone();
                newtree.push_all_move(tree2);
                out.push( (newtree, rem2) );
            }
        }
        out
    }
}

