use super::{HashMap, MoveItems};
use std::hash::Hash;
use std::iter::Chain;

fn append_move<T>(mut v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
    v1.push_all_move(v2);
    v1
}


// IDEA: Use Skip iterator and turn the parse() input into an iterator of
// some sort. Maybe not fast enough?

// S is a stream of input symbols, T is some type representing parsed input
trait Parser<S, T> {
    fn parse(&self, state: S) -> Results<T, S>;
}

struct EmptyIter;
impl<T> Iterator<T> for EmptyIter {
    fn next(&mut self) -> Option<T> { None }
}

enum Results<T, S> {
    ResultEmpty(EmptyIter),
    ResultItems(MoveItems<(T, S)>),
    ResultChain(Box<Chain<Results<T, S>, Results<T, S>>>),
}

impl<T, S> Iterator<(T, S)> for Results<T, S> {
    fn next(&mut self) -> Option<(T, S)> {
        match *self {
            ResultEmpty(ref mut it) => it.next(),
            ResultItems(ref mut it) => it.next(),
            ResultChain(ref mut it) => it.next(),
        }
    }
}



pub struct NilParser;

impl<T, S> Parser<S, Vec<T>> for NilParser {
    fn parse(&self, state: S) -> Results<Vec<T>, S> {
        let res = (vec!(), state);
        let vec = vec!(res);
        ResultItems( vec.move_iter() )
    }
}

pub struct SymParser<A> {
    sym: A,
}

// this is where having the input be an iterator would be very nice.
impl<'a, A: Eq + Clone> Parser<&'a [A], Vec<A>> for SymParser<A> {
    fn parse(&self, state: &'a [A]) -> Results<Vec<A>, &'a [A]> {
        match state.get(0) {
            None => ResultEmpty(EmptyIter),
            Some(sym) => {
                if *sym == self.sym {
                    let res = (vec!(self.sym.clone()), state.tailn(1));
                    let vec = vec!(res);
                    ResultItems( vec.move_iter() )
                } else {
                    ResultEmpty(EmptyIter)
                }
            },
        }

    }
}


pub struct AltParser<P, Q> {
    p1: P,
    p2: Q,
}

impl<S: Copy, T, P: Parser<S, T>, Q: Parser<S, T>> 
Parser<S, T> for AltParser<P, Q> {
    fn parse(&self, state: S) -> Results<T, S> {
        ResultChain(box self.p1.parse(state).chain(self.p2.parse(state)))
    }
}


pub fn alt<S, T, P: Parser<S, T>, Q: Parser<S, T>>(p1: P, p2: Q) -> AltParser<P, Q> {
    AltParser { p1: p1, p2: p2 }
}


/*
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


#[deriving(Clone)]
pub struct OptionalParser<P> {
    p: P,
}

impl<P> OptionalParser<P> {
    pub fn new(p: P) -> OptionalParser<P> {
        OptionalParser { p: p }
    }
}


impl<'a, S: Hash + Eq,
         I: ResultIter<Vec<S>, &'a [S]>,
         P: SimpleParser<'a, S, I> + Clone> Parser<&'a [S], Vec<S>, AltResultIter<StdResultIter<'a, S>, I>> for OptionalParser<P> {
    fn parse(&self, state: &'a [S]) -> AltResultIter<StdResultIter<'a, S>, I> {
        AltParser::new(NilParser, self.p.clone()).parse(state)
    }
}

*/
