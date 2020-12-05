// Rust doesn't have a way to chain "sort" methods (yet):
// https://github.com/rust-lang/rfcs/issues/2731
// so for now, let's patch the Vec with a .sorted:
//
// Note that we can alternatively use the `itertools` crate: https://docs.rs/itertools/0.9.0/itertools/fn.sorted.html

pub trait VecExt {
    fn sorted(self) -> Self;
}

impl<T> VecExt for Vec<T>
where
    T: std::cmp::Ord,
{
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}
