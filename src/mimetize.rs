use std::{collections::HashMap, hash::Hash};

pub struct Mimetize<I: Hash + Eq + Clone, O: Clone, F: Fn(I) -> O> {
    args: HashMap<I, O>,
    f: Box<F>,
}

impl<I: Hash + Eq + Clone, O: Clone, F: Fn(I) -> O> Mimetize<I, O, F> {
    pub fn new(f: F) -> Self {
        Self {
            args: HashMap::new(),
            f: Box::new(f),
        }
    }

    pub fn call(&mut self, i: I) -> O {
        match self.args.get(&i) {
            None => {
                let res = (self.f)(i.clone());
                self.args.insert(i, res.clone());
                res
            }
            Some(o) => o.clone(),
        }
    }
}
