use std::ops::{Add, Sub};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IntMap<T> {
    pub cells: Vec<T>,
    pub width: usize,
    pub height: usize,
}

#[expect(unused)]
impl<T: Default + Clone> IntMap<T> {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> IntMap<T> {
        let cells = vec![T::default(); width * height];
        IntMap {
            cells,
            width,
            height,
        }
    }

    pub fn from_raw<F: Fn(char) -> T>(input: &str, mapper: F) -> IntMap<T> {
        let width = input
            .trim()
            .replace('\r', "")
            .find('\n')
            .expect("must be row-wrapped with newlines");
        let data: Vec<T> = input
            .trim()
            .replace(['\n', '\r', ' ', '\t'], "")
            .chars()
            .map(mapper)
            .collect();

        let height = data.len() / width;

        assert_eq!(data.len(), width * height);
        IntMap {
            cells: data,
            width,
            height,
        }
    }

    pub fn in_bounds(&self, co: (isize, isize)) -> bool {
        0 <= co.0 && co.0 < self.width as isize && 0 <= co.1 && co.1 < self.height as isize
    }

    pub fn pt_to_idx(&self, co: (isize, isize)) -> isize {
        co.1 * self.width as isize + co.0
    }

    pub fn idx_to_pt(&self, idx: isize) -> (isize, isize) {
        (idx % self.width as isize, idx / self.width as isize)
    }

    pub fn get(&self, co: (isize, isize)) -> Option<&T> {
        if !self.in_bounds(co) {
            return None;
        }
        self.cells.get(self.pt_to_idx(co) as usize)
    }

    pub fn iter<'a>(&'a self) -> CoordIterator<'a, T> {
        CoordIterator {
            m: self,
            x: 0,
            y: 0,
        }
    }

    #[allow(dead_code)]
    pub fn set(&mut self, co: (isize, isize), val: T) {
        let idx = self.pt_to_idx(co);
        self.cells[idx as usize] = val;
    }
}

pub struct CoordIterator<'a, T> {
    m: &'a IntMap<T>,
    x: isize,
    y: isize,
}

impl<'a, T: Clone + Default> Iterator for CoordIterator<'a, T> {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.m.in_bounds((self.x, self.y)) {
            let out = Some((self.x, self.y));
            self.x += 1;
            return out;
        }

        self.x = 0;
        self.y += 1;

        if self.m.in_bounds((self.x, self.y)) {
            let out = Some((self.x, self.y));
            self.x += 1;
            return out;
        }

        None
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Coord<T> {
    x: T,
    y: T,
}

#[allow(unused)]
impl<T> Coord<T>
where
    T: Copy,
{
    pub fn get(&self) -> (T, T) {
        (self.x, self.y)
    }

    pub fn make(t: (T, T)) -> Self {
        Self { x: t.0, y: t.1 }
    }

    pub fn xy(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<Output = T>> Add for Coord<T>
where
    T: Add + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Coord<T>
where
    T: Sub + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Default for Coord<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
