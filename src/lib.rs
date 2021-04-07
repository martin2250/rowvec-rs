use std::ops::*;

#[cfg(test)]
mod test;

pub struct RowVec<T> {
    columns: usize,
    data: Vec<T>,
}

impl<T> RowVec<T> {
    pub fn new(columns: usize) -> Self {
        assert!(columns != 0);
        RowVec {
            columns: columns,
            data: Vec::new(),
        }
    }

    pub fn with_capacity(columns: usize, capacity: usize) -> Self {
        assert!(columns != 0);
        RowVec {
            columns: columns,
            data: Vec::with_capacity(columns * capacity),
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional * self.columns);
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn rows(&self) -> usize {
        debug_assert!(self.data.len() % self.columns == 0);
        self.data.len() / self.columns
    }

    pub fn append(&mut self, other: &mut Self) {
        debug_assert!(self.data.len() % self.columns == 0);
        assert!(self.columns == other.columns);
        self.data.append(&mut other.data);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> std::slice::Chunks<'_, T> {
        debug_assert!(self.data.len() % self.columns == 0);
        self.data.chunks(self.columns)
    }

    pub fn remove_range<R>(&mut self, range: R) 
    where R: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            Bound::Included(i) => Bound::Included(i*self.columns),
            Bound::Excluded(_) => todo![],
            Bound::Unbounded => Bound::Unbounded,
        };
        let end = match range.end_bound() {
            Bound::Included(i) => Bound::Excluded((i + 1)*self.columns),
            Bound::Excluded(i) => Bound::Excluded(i*self.columns),
            Bound::Unbounded => Bound::Unbounded,
        };
        self.data.drain((start,end));
    }

    pub fn iter_mut(&mut self) -> std::slice::ChunksMut<'_, T> {
        debug_assert!(self.data.len() % self.columns == 0);
        self.data.chunks_mut(self.columns)
    }

    pub fn swap_nonoverlapping(&mut self, a: usize, b: usize) {
        debug_assert!(self.data.len() % self.columns == 0);
        assert!(a != b);
        let i_l = a.min(b) * self.columns;
        let i_r = a.max(b) * self.columns;
        let (left, right) = self.data.split_at_mut(i_r);
        left[i_l..i_l+self.columns].swap_with_slice(&mut right[..self.columns]);
    }
}

impl<T: Clone> RowVec<T> {
    pub fn push(&mut self, value: &[T]) {
        debug_assert!(self.data.len() % self.columns == 0);
        assert!(self.columns == value.len());
        self.data.extend_from_slice(value);
    }
}

impl<T> Index<usize> for RowVec<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(self.data.len() % self.columns == 0);
        let i = index * self.columns;
        &self.data[i..i+self.columns]
    }
}

impl<T> IndexMut<usize> for RowVec<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
        debug_assert!(self.data.len() % self.columns == 0);
        let i = index * self.columns;
        &mut self.data[i..i+self.columns]
    }
}

impl<T> Index<(usize, usize)> for RowVec<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        debug_assert!(self.data.len() % self.columns == 0);
        assert!(index.1 < self.columns);
        &self.data[self.columns*index.0 + index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for RowVec<T> {
    fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut Self::Output {
        debug_assert!(self.data.len() % self.columns == 0);
        assert!(index.1 < self.columns);
        &mut self.data[self.columns*index.0 + index.1]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RowSlice<'a, T> {
    columns: usize,
    data: &'a [T],
}

impl<'a, T> RowVec<T> {
    pub fn slice(&'a self) -> RowSlice<'a, T> {
        RowSlice {
            columns: self.columns,
            data: &self.data,
        }
    }
}

impl <'a, T> RowSlice<'a, T> {
    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn rows(&self) -> usize {
        debug_assert!(self.data.len() % self.columns == 0);
        self.data.len() / self.columns
    }

    pub fn iter(&self) -> std::slice::Chunks<'_, T> {
        debug_assert!(self.data.len() % self.columns == 0);
        self.data.chunks(self.columns)
    }

    pub fn range<R>(&self, range: R) -> RowSlice<'a, T>
    where R: RangeBounds<usize>,
     {
        let start = match range.start_bound() {
            Bound::Included(i) => i*self.columns,
            Bound::Excluded(i) => (i+1)*self.columns,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(i) => (i + 1)*self.columns,
            Bound::Excluded(i) => i*self.columns,
            Bound::Unbounded => self.data.len(),
        };
        RowSlice {
            columns: self.columns,
            data: &self.data[std::ops::Range{start: start, end: end}],
        }
    }

    pub fn range_from(&self, start: usize) -> RowSlice<'a, T> {
        let s = start * self.columns;
        RowSlice {
            columns: self.columns,
            data: &self.data[s..],
        }
    }

    pub fn range_to(&self, end: usize) -> RowSlice<'a, T> {
        let e = end * self.columns;
        RowSlice {
            columns: self.columns,
            data: &self.data[..e],
        }
    }
}

impl<'a, T> Index<usize> for RowSlice<'a, T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(self.data.len() % self.columns == 0);
        let i = index * self.columns;
        &self.data[i..i+self.columns]
    }
}

impl<'a, T> Index<(usize, usize)> for RowSlice<'a, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        debug_assert!(self.data.len() % self.columns == 0);
        assert!(index.1 < self.columns);
        &self.data[self.columns*index.0 + index.1]
    }
}
