use super::{HashMap, MoveItems};
use std::hash::Hash;
use std::iter::{Chain, FlatMap};

fn append_move<T>(mut v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
    v1.push_all_move(v2);
    v1
}


// IDEA: Use Skip iterator and turn the parse() input into an iterator of
// some sort. Maybe not fast enough?

// S is a stream of input symbols, T is some type representing parsed input
pub trait Parser<'a, S, T> {
    fn parse(&self, state: S) -> Results<'a, T, S>;
}

struct EmptyIter;
impl<T> Iterator<T> for EmptyIter {
    fn next(&mut self) -> Option<T> { None }
}

enum Results<'a, T, S> {
    ResultEmpty(EmptyIter),
    ResultItems(MoveItems<(T, S)>),
    ResultChain(Box<Chain<Results<'a, T, S>, Results<'a, T, S>>>),
    ResultFlatMap(Box<FlatMap<'a, (T, S), Results<'a, T, S>, Results<'a, T, S>>>),
}

impl<'a, T, S> Iterator<(T, S)> for Results<'a, T, S> {
    fn next(&mut self) -> Option<(T, S)> {
        match *self {
            ResultEmpty(ref mut it) => it.next(),
            ResultItems(ref mut it) => it.next(),
            ResultChain(ref mut it) => it.next(),
            ResultFlatMap(ref mut it) => it.next(),
        }
    }
}



struct NilParser;

impl<'a, T, S> Parser<'a, S, Vec<T>> for NilParser {
    fn parse(&self, state: S) -> Results<Vec<T>, S> {
        let res = (vec!(), state);
        let vec = vec!(res);
        ResultItems( vec.move_iter() )
    }
}

pub struct SymParser<A> {
    sym: A,
}

impl<A> SymParser<A> {
    pub fn new(x: A) -> SymParser<A> {
        SymParser { sym: x }
    }
}

// this is where having the input be an iterator would be very nice.
impl<'a, 'b, A: Eq + Clone> Parser<'b, &'a [A], Vec<A>> for SymParser<A> {
    fn parse(&self, state: &'a [A]) -> Results<'b, Vec<A>, &'a [A]> {
        match state.get(0) {
            None => ResultEmpty(EmptyIter),
            Some(sym) => {
                if *sym == self.sym {
                    let res = (vec!(sym.clone()), state.tailn(1));
                    let vec = vec!(res);
                    ResultItems( vec.move_iter() )
                } else {
                    ResultEmpty(EmptyIter)
                }
            },
        }

    }
}


struct DotParser;

impl<'a, 'b, A: Clone> Parser<'b, &'a [A], Vec<A>> for DotParser {
    fn parse(&self, state: &'a [A]) -> Results<'b, Vec<A>, &'a [A]> {
        match state.get(0) {
            None => ResultEmpty(EmptyIter),
            Some(sym) => {
                let res = (vec!(sym.clone()), state.tailn(1));
                let vec = vec!(res);
                ResultItems( vec.move_iter() )
            },
        }

    }
}


struct AltParser<P, Q> {
    p1: P,
    p2: Q,
}

impl<'a, S: Copy, T, P: Parser<'a, S, T>, Q: Parser<'a, S, T>> 
Parser<'a, S, T> for AltParser<P, Q> {
    fn parse(&self, state: S) -> Results<'a, T, S> {
        ResultChain(box self.p1.parse(state).chain(self.p2.parse(state)))
    }
}

struct CatParser<P, Q> {
    p1: P,
    p2: Q,
}

impl<'a, S: Copy, T, P: Parser<'a, S, T>, Q: Parser<'a, S, T>> 
Parser<'a, S, T> for CatParser<P, Q> {
    fn parse(&self, state: S) -> Results<'a, T, S> {
        ResultFlatMap( box self.p1.parse(state).flat_map(|x| self.p2.parse(x.val1())) )
    }
}

/*
// The idea is that I is an iterator and P is a parser.
struct ConcatResults<S, T, P> {
    iter: Results<T, S>, // results from first parse
    p: P,
    iter2: Option<PrependResults<T, S>>, // the iterator from second parse
}

impl<S, T, P: Parser<S, Vec<T>>> Iterator<(Vec<T>, S)> for ConcatResults<S, Vec<T>, P> {
    fn next(&mut self) -> Option<(T, S)> {
        let mut n: Option<(Vec<T>, S)>;
        loop {
            if self.iter2.is_none() {
                match self.iter.next() {
                    None => return None,
                    Some((parsed, rem)) => {
                        self.iter2 = PrependResults { val: parsed, it: self.p.parse(rem) };
                    }
                }
            }

            n = self.iter2.next();
            if n.is_some() {
                break;
            }
        }

        n
    }
}
*/


// Main public functions

pub fn opt<'a, S, T, P: Parser<'a, S, T>>(p: P) -> AltParser<NilParser, P> {
    AltParser { p1: NilParser, p2: p }
}

pub fn alt<'a, S, T, P: Parser<'a, S, T>, Q: Parser<'a, S, T>>(p1: P, p2: Q) -> AltParser<P, Q> {
    AltParser { p1: p1, p2: p2 }
}

pub fn dot() -> DotParser {
    DotParser
}

pub fn nil() -> NilParser {
    NilParser
}

pub fn cat<'a, S, T, P: Parser<'a, S, T>, Q: Parser<'a, S, T>>(p1: P, p2: Q) -> CatParser<P, Q> {
    CatParser { p1: p1, p2: p2 }
}

/*
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
                Some((parsed, rem)) => {
                    return Some( (append_move( self.init_parsed.clone(), parsed),
                           rem));
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

*/
