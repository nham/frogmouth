use super::{HashMap, MoveEntries, Parser};
use std::hash::Hash;
use std::iter::Chain;

fn append_move<T>(mut v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
    v1.push_all_move(v2);
    v1
}

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
for AltResultIter<StdResultIter<'a, S>, StdResultIter<'a, S>> {}

impl<'a, S: Clone, 
         I: ResultIter<Vec<S>, &'a [S]>, 
         J: ResultIter<Vec<S>, &'a [S]>,
         P: SimpleParser<'a, S, J>> 
    ResultIter<Vec<S>, &'a [S]> for ConcatResultIter<S, I, J, P> {}


struct AltResultIter<I, J> {
    chain: Chain<I, J>,
}

impl<'a, S, 
         I: ResultIter<Vec<S>, &'a [S]>, 
         J: ResultIter<Vec<S>, &'a [S]>>
Iterator<(Vec<S>, &'a [S])>
for AltResultIter<I, J> {
    fn next(&mut self) -> Option<(Vec<S>, &'a [S])> {
        self.chain.next()
    }
}



// The idea is that I is an iterator and P is a parser.
struct ConcatResultIter<S, I, J, P> {
    iter: I,
    p: P,
    init_parsed: Vec<S>, // the parsed vector after initial parse
    iter2: Option<J>, // the iterator from second parse
}

impl<T, S, I, J: Iterator<T>, P> ConcatResultIter<S, I, J, P> {
    fn next_iter2(&mut self) -> Option<T> {
        match self.iter2 {
            None => None,
            Some(ref mut it) => it.next(),
        }
    }
}


impl<'a, S: Clone, 
         I: ResultIter<Vec<S>, &'a [S]>, 
         J: ResultIter<Vec<S>, &'a [S]>,
         P: SimpleParser<'a, S, J>>
Iterator<(Vec<S>, &'a [S])>
for ConcatResultIter<S, I, J, P> {
    fn next(&mut self) -> Option<(Vec<S>, &'a [S])> {
        if self.iter2.is_some() {
            let next_iter2 = self.next_iter2();
            match next_iter2 {
                Some(x) => {
                    // TODO
                    return None;
                },
                None => {},
            }
        }

        // If we make it here we need to try to get a non-exhausted iter2.
        match self.iter.next() {
            None => None, // nothing in self.iter either, bail
            Some((parsed, rem)) => {
                self.init_parsed = parsed;
                let new_iter = self.p.parse(rem);
                self.iter2 = Some(new_iter);

                match self.next_iter2() {
                    None => None,
                    Some((parsed2, rem2)) => {
                        Some((append_move( self.init_parsed.clone(), parsed2 ),
                              rem2))
                    },
                }
            }
        }
    }
}



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


#[deriving(Clone)]
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


#[deriving(Clone)]
pub struct AltParser<P, Q> {
    p1: P,
    p2: Q,
}

impl<P, Q> AltParser<P, Q> {
    pub fn new(p1: P, p2: Q) -> AltParser<P, Q> {
        AltParser { p1: p1, p2: p2 }
    }
}


impl<'a, S, 
         I: ResultIter<Vec<S>, &'a [S]>,
         J: ResultIter<Vec<S>, &'a [S]>,
         P: SimpleParser<'a, S, I>,
         Q: SimpleParser<'a, S, J>> 
    Parser<&'a [S], Vec<S>, AltResultIter<I, J>> for AltParser<P, Q> {
    fn parse(&self, state: &'a [S]) -> AltResultIter<I, J> {
        AltResultIter { chain: self.p1.parse(state).chain(self.p2.parse(state)) }
    }
}


#[deriving(Clone)]
pub struct ConcatParser<P, Q> {
    p1: P,
    p2: Q,
}

impl<P, Q> ConcatParser<P, Q> {
    pub fn new(p1: P, p2: Q) -> ConcatParser<P, Q> {
        ConcatParser { p1: p1, p2: p2 }
    }
}


// This is a bad because we require Q to implement Clone. I think ideally
// ConcatResultIter should only hold a reference to a Parser. However, that
// means it has to take a lifetime parameter, so far I've been unable to make
// the borrow checker agree with what I've written.
impl<'a, S: Clone, 
         I: ResultIter<Vec<S>, &'a [S]>,
         J: ResultIter<Vec<S>, &'a [S]>,
         P: Parser<&'a [S], Vec<S>, I>,
         Q: Parser<&'a [S], Vec<S>, J> + Clone> 
    Parser<&'a [S], Vec<S>, ConcatResultIter<S, I, J, Q>> for ConcatParser<P, Q> {
    fn parse(&self, state: &'a [S]) -> ConcatResultIter<S, I, J, Q> {
        let mut it = self.p1.parse(state);
        ConcatResultIter { iter: it, p: self.p2.clone(), init_parsed: vec!(), iter2: None }

    }
}
