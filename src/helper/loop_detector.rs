use std::mem;

/* -------------------------------------------------------------------------- */

/// # Usage
/// ```
/// # fn initial_state(): () {}
/// # fn do_cycle(state: &mut ()) {}
///
/// const CYCLE_COUNT: usize = 1_000_000_000;
///
/// let mut cycle_detector = CycleDetector::new();
///
/// let mut state = initial_state();
/// cycle_detector.insert(state.clone());
///
/// for _ in 0..CYCLE_COUNT {
///     do_cycle(&mut state);
///     if cycle_detector.insert(state.clone()) {
///         let the_loop = cycle_detector.into_loop().ok().unwrap();
///         *state = the_loop.get(CYCLE_COUNT).clone();
///         break;
///     }
/// }
/// ```
pub struct LoopDetector<T>(Detection<T>);

enum Detection<T> {
    Loop(Loop<T>),
    NoLoop { states: Vec<T> },
}

impl<T> Default for Detection<T> {
    fn default() -> Self {
        Detection::NoLoop { states: Vec::new() }
    }
}

impl<T> Default for LoopDetector<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LoopDetector<T> {
    pub fn new() -> Self {
        Self(Detection::NoLoop { states: Vec::new() })
    }
}

impl<T> LoopDetector<T>
where
    T: Eq,
{
    pub fn insert(&mut self, state: T) -> bool {
        self.0 = match mem::take(&mut self.0) {
            // TODO: use Result
            Detection::Loop(_) => panic!("loop already found !"),
            Detection::NoLoop { mut states } => {
                let loop_start = states
                    .iter()
                    .enumerate()
                    .find_map(|(i, x)| (x == &state).then_some(i));

                if let Some(loop_start) = loop_start {
                    // The state already exists
                    let the_loop = Loop {
                        values: states.into_boxed_slice(),
                        loop_start,
                    };

                    Detection::Loop(the_loop)
                } else {
                    // The new state has been added
                    states.push(state);

                    Detection::NoLoop { states }
                }
            }
        };

        matches!(self.0, Detection::Loop(_))
    }

    pub fn into_loop(self) -> Result<Loop<T>, Self> {
        match self.0 {
            Detection::Loop(the_loop) => Ok(the_loop),
            Detection::NoLoop { .. } => Err(self),
        }
    }
}

/* -------------------------------------------------------------------------- */

/// ```txt
///                  LEN
///   <─────────────────────────────────>
///                        LOOP LEN
///                    <───────────────>
///   [0] ──> [1] ──> [2] ──> [3] ──> [4]
///                    ^               │
///                    └───────────────┘
///                    └ LOOP START
/// ```
pub struct Loop<T> {
    values: Box<[T]>,
    loop_start: usize,
}

impl<T> Loop<T> {
    #[inline]
    pub fn loop_start(&self) -> usize {
        self.loop_start
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    #[inline]
    pub fn loop_len(&self) -> usize {
        self.values.len() - self.loop_start
    }

    #[inline]
    pub fn values(&self) -> &[T] {
        &self.values
    }

    #[inline]
    pub fn loop_values(&self) -> &[T] {
        &self.values[self.loop_start..]
    }

    /// Returns the elements at the index `i` from this loop as an infinite slice.
    ///
    /// ```txt
    ///   [0, 1, 2, 3, 4, 2, 3, 4, 2, 3, 4, ...]
    ///
    ///   [0] ──> [1] ──> [2] ──> [3] ──> [4]
    ///                    ^               │
    ///                    └───────────────┘
    /// ```
    #[inline]
    pub fn get(&self, i: usize) -> &T {
        if let Some(i) = i.checked_sub(self.loop_start) {
            let i = i % self.loop_len();
            let i = i + self.loop_start;
            &self.values[i]
        } else {
            &self.values[i]
        }
    }
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::LoopDetector;

    #[test]
    fn loop_detector() {
        let mut loop_detector = LoopDetector::new();

        assert!(!loop_detector.insert(0));
        assert!(!loop_detector.insert(1));
        assert!(!loop_detector.insert(2));
        assert!(!loop_detector.insert(3));
        assert!(!loop_detector.insert(4));
        assert!(loop_detector.insert(2));

        let the_loop = loop_detector.into_loop().ok().unwrap();
        assert_eq!(the_loop.values(), &[0, 1, 2, 3, 4]);
        assert_eq!(the_loop.loop_start(), 2);
    }
}

/* -------------------------------------------------------------------------- */
