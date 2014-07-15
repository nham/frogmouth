// My attempt at FlatMap, but with using a trait method rather than closures
pub struct TraitFlatMap<I, J, G> {
    iter: I,
    g: G,
    iter2: Option<J>,
}

impl<A, I: Iterator<A>, B, J: Iterator<B>, G: IterGen<A, B, J>> 
Iterator<B> for TraitFlatMap<I, J, G> {
    fn next(&mut self) -> Option<B> {
        loop {
            // if iter2 has something, we need to iterate it
            // until it's exhausted.
            // if iter2 has nothing, we should take an element
            // from iter and use it to generate iter2.
            // if we are unable to take from iter, return None.
            if self.iter2.is_none() {
                match self.iter.next() {
                    None => return None,
                    Some(x) => {
                        self.iter2 = Some(self.g.gen(x))
                    }
                }
            }

            let n = self.iter2.get_mut_ref().next();
            if n.is_some() {
                return n;
            } else {
                self.iter2 = None;
            }
        }
    }
}

pub trait IterGen<A, B, T: Iterator<B>> {
    fn gen(&mut self, x: A) -> T;
}


mod test {
    use super::{TraitFlatMap, IterGen};
    use std::iter::{Take, Counter, count};

    #[test]
    fn test_it() {
        struct Foo {
            s: int,
        }

        impl IterGen<uint, int, Take<Counter<int>>> for Foo {
            fn gen(&mut self, x: uint) -> Take<Counter<int>> {
                let tmp = self.s;
                self.s += 1;
                count(tmp, 1).take(x)
            }
        }

        let mut foo = Foo { s: -7 };

        let mut fm: TraitFlatMap<Take<Counter<uint>>, Take<Counter<int>>, Foo>
            = TraitFlatMap { iter: count(3u, 2).take(4),
                             g: foo,
                             iter2: None, };

        let ys = [-7, -6, -5,  -6, -5, -4, -3, -2, 
                  -5, -4, -3, -2, -1, 0, 1,  -4, -3, -2, -1, 0, 1, 2, 3, 4i];
        for (x, y) in fm.zip(ys.iter()) {
            assert!(x == *y);
        }
    }
}
