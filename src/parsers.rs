use super::{HashMap, MoveEntries, Parser};
use std::hash::Hash;
use std::iter::Chain;

// Parser where the input stream type is a slice of S's, and the 
// "parsed" representation is a vector of S's
trait SimpleParser<'a, S, I>: Parser<&'a [S], Vec<S>, I> {}

impl<'a, S, I, P: Parser<&'a [S], Vec<S>, I>> 
    SimpleParser<'a, S, I> for P {}

type StdResultIter<'a, S> = MoveEntries<Vec<S>, &'a [S]>;


// Essentially a dummy trait to unify the different varieties
// of iterators that I use
trait ResultIter<A,B>: Iterator<(A,B)> {}

impl<'a, S> ResultIter<Vec<S>, &'a [S]> for StdResultIter<'a, S> {}
impl<'a, S> ResultIter<Vec<S>, &'a [S]> 
for Chain<StdResultIter<'a, S>, StdResultIter<'a, S>> {}


// The idea is that I is an iterator and P is a parser.
struct ConcatResultIter<S, I, J, P> {
    iter: I,
    p: P,
    init_parsed: Vec<S>, // the parsed vector after initial parse
    p_iter: Option<J>, // the iterator from second parse
}

impl<S, I, J, P> ConcatResultIter<S, I, J, P> {
    fn append_init_parsed(&self,
}


fn blarg(vec: &Vec<S>, mut tup: (Vec<S>, &'a [S])) -> (Vec<S>, &'a [S]) {
    let (consumed, remaining) = tup;
    let newvec = vec.clone();
    newvec.push_all_move(consumed);
    (newvec, remaining)
}

impl<'a, S, I: ResultIter<Vec<S>, &'a [S]>, 
            J: ResultIter<Vec<S>, &'a [S]>,
            P: SimpleParser<'a, S, J>>
Iterator<(Vec<S>, &'a [S])>
for ConcatResultIter<S, I, J, P> {
    fn next(&mut self) -> Option<(Vec<S>, &'a [S])> {
        let p_iter_next = self.p_iter;

        let new_iter: J;

        if self.p_iter.is_some() {
            new_iter = self.p_iter.unwrap().next
        }

        if self.p_iter.is_none() || self.p_iter.unwrap().ne{
            match self.iter.next() {
                None => None,
                Some((parsed, rem)) => {
                    self.init_parsed = parsed;
                    let new_iter = self.p.parse(rem);
                    self.p_iter = Some(new_iter);

                    match self.p_iter.next()
                        None => None,
                        Some((parsed2, rem2)) => None,
                    }
                }
            }
        } else {
            p_iter_next.unwrap()
        }
    }
}

impl<'a, S, I: ResultIter<Vec<S>, &'a [S]>, 
            J: ResultIter<Vec<S>, &'a [S]>,
            P: SimpleParser<'a, S, J>> 
    ResultIter<Vec<S>, &'a [S]> for ConcatResultIter<S, I, J, P> {}


/****************************/


pub struct NilParser;

impl NilParser {
    pub fn new() -> NilParser {
        NilParser
    }
}

impl<'a, S: Hash + Eq> Parser<&'a S, Vec<S>, StdResultIter<'a, S>> 
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
         P: SimpleParser<'a, S, I>,
         Q: SimpleParser<'a, S, J>> 
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


impl<'a, S: Hash + Eq, 
         I: ResultIter<Vec<S>, &'a [S]>,
         J: ResultIter<Vec<S>, &'a [S]>,
         P: Parser<&'a [S], Vec<S>, I>,
         Q: Parser<&'a [S], Vec<S>, J>> 
    Parser<&'a [S], Vec<S>, ConcatResultIter<S, I, J, Q>> for ConcatParser<P, Q> {
    fn parse(&self, state: &'a [S]) -> ConcatResultIter<S, I, J, Q> {
        let p1_parse = self.p1.parse(state);

    }
}
