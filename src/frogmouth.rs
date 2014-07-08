use tree::{Tree, Node};

mod tree;

type ParseResult<'a, S, T> = Vec<(&'a [S], T)>;

trait Parser<S,T> {
    fn parse<'a>(&self, state: &'a [S]) -> ParseResult<'a, S, T>;
}

struct altParser<P, Q> {
    p1: P,
    p2: Q,
}

impl<S,T, P: Parser<S,T>, Q: Parser<S,T>> Parser<S,T> for altParser<P, Q> {
    fn parse<'a>(&self, state: &'a [S]) -> ParseResult<'a, S, T> {
        let p1_parse = self.p1.parse(state);
        if p1_parse.len() > 0 {
            p1_parse
        } else {
            self.p2.parse(state)
        }
    }
}


struct symParser {
    sym: char,
}

impl Parser<char, Tree<char>> for symParser {
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

}
