/* -------------------------------------------------------------------------- */

/// Represents a succession of values with a cycle.
///
/// This cycle acts like an infinite array of value.
///
/// ```txt
///                 VALUES
///   <─────────────────────────────────>
///    HEAD PART           CYCLE PART
///    <───────>       <───────────────>
///   [0] ──> [1] ──> [2] ──> [3] ──> [4]
///                    ^               │
///                    └───────────────┘
///                    │
///                    └ CYCLE START
///
///   [0, 1, 2, 3, 4, 2, 3, 4, 2, 3, 4, ...]
/// ```
// NOTE: `Cycle` didn't implement `Default` because an empty cycle will violate the invariant on `cycle_start`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cycle<T> {
    values: Box<[T]>,
    // INVARIANT: `cycle_start` < `values.len`
    cycle_start: usize,
}

impl<T> Cycle<T> {
    /// Returns the index of the first element of the cycle part.
    #[inline]
    pub fn cycle_start(&self) -> usize {
        self.cycle_start
    }

    /// Returns the number of elements.
    #[inline]
    // NOTE: `Cycle` is never empty.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns a slice over all the values.
    #[inline]
    pub fn values(&self) -> &[T] {
        &self.values
    }

    /// Returns a slice over all the values in the head part.
    #[inline]
    pub fn head(&self) -> &[T] {
        &self.values[..self.cycle_start]
    }

    /// Returns a slice over all the values in the cycle part.
    #[inline]
    pub fn cycle(&self) -> &[T] {
        &self.values[self.cycle_start..]
    }

    /// Returns a slice over the head part and a slice over the cycle part.
    #[inline]
    pub fn split(&self) -> (&[T], &[T]) {
        self.values.split_at(self.cycle_start)
    }

    /// An *infinite* iterator over the values of this cycle.
    ///
    /// Iterates over the element of the head part then over the elements
    /// of the cycle part repeatedly.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let (head, cycle) = self.split();
        head.iter().chain(cycle.iter().cycle())
    }

    /// Converts the index to match an element of this cycle.
    fn convert_index(&self, i: usize) -> usize {
        if let Some(i) = i.checked_sub(self.cycle_start) {
            let i = i % self.cycle().len();
            i + self.cycle_start
        } else {
            i
        }
    }

    /// Returns a reference to an element at the index `i`.
    #[inline]
    pub fn get(&self, i: usize) -> &T {
        &self.values[self.convert_index(i)]
    }

    /// Returns an element at the index `i`.
    #[inline]
    pub fn get_into(self, i: usize) -> T {
        let i = self.convert_index(i);
        let mut v = self.values.into_vec();
        v.swap_remove(i)
    }
}

/* -------------------------------------------------------------------------- */

/// Computes all values of `init_state` after repeatedly applying `compute_next_state`
/// for `cycle_count` times.
///
/// If a cycle is detected returns `Ok(cycle)` otherwise returns a `Vec` with all computed values.
///
/// `compute_next_state` must be pure, i.e. the returned value depends only of the input value,
/// otherwise this function may returns false positive.
pub fn detect_cycle<T>(
    cycle_count: usize,
    init_state: T,
    mut compute_next_state: impl FnMut(&T) -> T,
) -> Result<Cycle<T>, Vec<T>>
where
    T: Eq,
{
    let mut states = vec![init_state];

    for _ in 0..cycle_count {
        // SAFETY: `states` contains at least 1 element.
        let current_state = unsafe { states.last().unwrap_unchecked() };
        let next_state = compute_next_state(current_state);

        let cycle_start = states
            .iter()
            .enumerate()
            .find_map(|(i, x)| (x == &next_state).then_some(i));

        if let Some(cycle_start) = cycle_start {
            return Ok(Cycle {
                values: states.into_boxed_slice(),
                cycle_start,
            });
        } else {
            states.push(next_state);
        }
    }

    Err(states)
}

/// Computes the value of `init_state` after repeatedly applying `compute_next_state`
/// for `cycle_count` times, using a cycle detection strategy.
pub fn compute_last_state<T>(
    cycle_count: usize,
    init_state: T,
    compute_next_state: impl FnMut(&T) -> T,
) -> T
where
    T: Eq,
{
    match detect_cycle(cycle_count, init_state, compute_next_state) {
        Ok(cycle) => cycle.get_into(cycle_count),
        Err(mut states) => states.pop().unwrap(),
    }
}

/* -------------------------------------------------------------------------- */
