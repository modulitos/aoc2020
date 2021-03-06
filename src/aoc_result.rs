#[allow(clippy::doc_markdown)]
/// Some AoC puzzles return a `u32`, while others return a `Vec<u32`.  In deciding to have all
/// the puzzles run from a single application, it makes sense to harmonize these types to return
/// to the caller.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AocReturn {
    U32Item(u32),
    UsizeItem(usize),
    U64Item(u64),
    U32ItemOpt(Option<u32>),
    U32List(Vec<u32>),
    UsizeList(Vec<usize>),
}

impl From<Option<u32>> for AocReturn {
    fn from(item: Option<u32>) -> Self {
        Self::U32ItemOpt(item)
    }
}

impl From<u32> for AocReturn {
    fn from(item: u32) -> Self {
        Self::U32Item(item)
    }
}

impl From<u64> for AocReturn {
    fn from(item: u64) -> Self {
        Self::U64Item(item)
    }
}

impl From<usize> for AocReturn {
    fn from(item: usize) -> Self {
        Self::UsizeItem(item)
    }
}

impl From<Vec<u32>> for AocReturn {
    fn from(list: Vec<u32>) -> Self {
        Self::U32List(list)
    }
}

impl From<Vec<usize>> for AocReturn {
    fn from(list: Vec<usize>) -> Self {
        Self::UsizeList(list)
    }
}
