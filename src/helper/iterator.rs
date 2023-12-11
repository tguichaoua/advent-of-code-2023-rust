use std::iter::{Peekable, Skip};

pub struct UniquePair<I: Iterator> {
    iter: I,
    first: Peekable<I>,
    second: Skip<I>,
    to_skip: usize,
}

impl<I> UniquePair<I>
where
    I: Iterator + Clone,
{
    fn new(iter: I) -> Self {
        let to_skip = 1;
        let first = iter.clone().peekable();
        let second = iter.clone().skip(to_skip);
        Self {
            iter,
            first,
            second,
            to_skip,
        }
    }
}

impl<I> Iterator for UniquePair<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let mut first = self.first.peek()?;

        let second = loop {
            let second = self.second.next();

            if let Some(second) = second {
                break second;
            }

            let _ = self.first.next();
            first = self.first.peek()?;
            self.to_skip += 1;
            self.second = self.iter.clone().skip(self.to_skip);
        };

        Some((first.clone(), second))
    }
}

pub fn unique_pair<I>(iter: I) -> UniquePair<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    UniquePair::new(iter)
}
