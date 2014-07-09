use super::{Parser};
use std::collections::hashmap::{HashMap, MoveEntries};
use std::hash::Hash;
use std::iter::Chain;

type ParseResult<'a, S> = (Vec<S>, &'a [S]);

type StdResultIter<'a, S> = MoveEntries<Vec<S>, &'a [S]>;

fn apply_move<T>(mut v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
    v1.push_all_move(v2);
    v1
}

trait ResultIter<A,B>: Iterator<(A,B)> {}


impl<'a, S> ResultIter<Vec<S>, &'a [S]> for StdResultIter<'a, S> {}
impl<'a, S> ResultIter<Vec<S>, &'a [S]> 
    for Chain<StdResultIter<'a, S>, StdResultIter<'a, S>> {}


/****************************/


pub struct NilParser;

impl NilParser {
    pub fn new() -> NilParser {
        NilParser
    }
}

impl<'a, S: Hash + Eq> Parser<&'a [S], Vec<S>, StdResultIter<'a, S>> 
for NilParser {
    fn parse<'a>(&self, state: &'a [S]) -> StdResultIter<'a, S> {
        let mut hm = HashMap::new();
        hm.insert(vec!(), state);
        hm.move_iter()
    }
}


pub struct SymParser<S> {
    sym: S,
}

impl<S> SymParser<S> {
    pub fn new(c: S) -> SymParser<S> {
        SymParser { sym: c }
    }
}

impl<'a, S: Hash + Eq + Clone> Parser<&'a [S], Vec<S>, StdResultIter<'a, S>> 
for SymParser<S> {
    fn parse(&self, state: &'a [S]) -> StdResultIter<'a, S> {
        match state.get(0) {
            None => HashMap::new().move_iter(),
            Some(sym) => {
                if *sym == self.sym {
                    let mut hm = HashMap::new();
                    hm.insert(vec!( self.sym.clone() ), state.tailn(1));
                    hm.move_iter()
                } else {
                    HashMap::new().move_iter()
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


impl<'a, S: Hash + Eq, 
         I: ResultIter<Vec<S>, &'a [S]>,
         J: ResultIter<Vec<S>, &'a [S]>,
         P: Parser<&'a [S], Vec<S>, I>,
         Q: Parser<&'a [S], Vec<S>, J>> 
    Parser<&'a [S], Vec<S>, Chain<I, J>> for AltParser<P, Q> {
    fn parse(&self, state: &'a [S]) -> Chain<I, J> {
        self.p1.parse(state).chain(self.p2.parse(state))
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


/*
impl<'a, S: Hash + Eq, 
         P: Parser<&'a [S], Vec<S>, ResultIter<'a, S>>,
         Q: Parser<&'a [S], Vec<S>, ResultIter<'a, S>>> 
    Parser<&'a [S], Vec<S>, ResultIter<'a, S>> for ConcatParser<P, Q> {
    fn parse(&self, state: &'a [S]) -> ResultIter<'a, S> {
        let p1_parse = self.p1.parse(state);

        let mut out = vec!();
        for (con, rem) in p1_parse.move_iter() {
            for (con2, rem2) in self.p2.parse(rem).move_iter() {
                out.push( (apply_move(con.clone(), con2), rem2) );
            }
        }
        out
    }
}
*/
