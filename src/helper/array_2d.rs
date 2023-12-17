use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array2D<T> {
    values: Box<[T]>,
    width: usize,
}

impl<T> Default for Array2D<T> {
    fn default() -> Self {
        Self {
            values: Default::default(),
            width: 0,
        }
    }
}

impl<T> Array2D<T> {
    pub fn from_elem(value: T, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        let values = vec![value; width * height].into_boxed_slice();
        Self { values, width }
    }
}

impl<T> Array2D<T> {
    #[allow(clippy::should_implement_trait)]
    pub fn from_iter(iter: impl IntoIterator<Item = impl IntoIterator<Item = T>>) -> Self {
        let mut iter = iter.into_iter();

        let Some(first_line) = iter.next() else {
            return Self::default();
        };

        let mut values: Vec<T> = first_line.into_iter().collect();
        let width = values.len();

        values.extend(iter.flatten());

        // TODO: use Result instead of panic
        // FIXME: this check is not solid enough
        // e.g. if line lens are [3, 2, 4], this check will pass.
        assert!(
            values.len() % width == 0,
            "Array2D::from_iter: some lines have not the same len"
        );

        let values = values.into_boxed_slice();

        Self { values, width }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.values.len() / self.width
    }

    pub fn get(&self, x: usize, y: usize) -> Result<&T, OutOfBoundError> {
        if x >= self.width {
            return Err(OutOfBoundError::X);
        }
        let i = self.width * y + x;
        self.values.get(i).ok_or(OutOfBoundError::Y)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut T, OutOfBoundError> {
        if x >= self.width {
            return Err(OutOfBoundError::X);
        }
        let i = self.width * y + x;
        self.values.get_mut(i).ok_or(OutOfBoundError::Y)
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        // TODO: use Result
        assert!(x < self.width);
        let i = self.width * y + x;
        self.values[i] = value;
    }
}

impl<T> Array2D<T> {
    pub fn per_line(&self) -> PerLine<T> {
        PerLine {
            array: self,
            current: 0,
        }
    }
}

/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub enum OutOfBoundError {
    X,
    Y,
}

impl Display for OutOfBoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutOfBoundError::X => f.write_str("`x` index is out-of-bound"),
            OutOfBoundError::Y => f.write_str("`y` index is out-of-bound"),
        }
    }
}

impl Error for OutOfBoundError {}

/* -------------------------------------------------------------------------- */

pub struct PerLine<'a, T> {
    array: &'a Array2D<T>,
    current: usize,
}

impl<'a, T> Iterator for PerLine<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let range = (self.current * self.array.width)..((self.current + 1) * self.array.width);
        self.current += 1;
        self.array.values.get(range)
    }
}

/* -------------------------------------------------------------------------- */
