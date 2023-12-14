/* -------------------------------------------------------------------------- */

/// Represents a succession of values including a loop.
///
/// ```txt
///                  LEN
///   <─────────────────────────────────>
///                        LOOP LEN
///                    <───────────────>
///   [0] ──> [1] ──> [2] ──> [3] ──> [4]
///                    ^               │
///                    └───────────────┘
///                    │
///                    └ LOOP START
/// ```
// NOTE: `Loop` didn't implement `Default` because an empty loop will violate the invariant on `loop_start`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Loop<T> {
    values: Box<[T]>,
    // INVARIANT: `loop_start` < `values.len`
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

    /// An *infinite* iterator over the values of the loop.
    ///
    /// ```txt
    ///   [0, 1, 2, 3, 4, 2, 3, 4, 2, 3, 4, ...]
    ///
    ///   [0] ──> [1] ──> [2] ──> [3] ──> [4]
    ///                    ^               │
    ///                    └───────────────┘
    /// ```
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let (head, the_loop) = self.values.split_at(self.loop_start);
        head.iter().chain(the_loop.iter().cycle())
    }

    /// Converts the index to match an element of this loop.
    ///
    /// ```txt
    ///   input:  [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, ...]
    ///            V  V  V  V  V  V  V  V  V  V   V
    ///   output: [0, 1, 2, 3, 4, 2, 3, 4, 2, 3,  4, ...]
    ///
    ///   [0] ──> [1] ──> [2] ──> [3] ──> [4]
    ///                    ^               │
    ///                    └───────────────┘
    /// ```
    fn convert_index(&self, i: usize) -> usize {
        if let Some(i) = i.checked_sub(self.loop_start) {
            let i = i % self.loop_len();
            i + self.loop_start
        } else {
            i
        }
    }

    /// Returns a reference to the elements at the index `i` from this loop as an infinite slice.
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
        &self.values[self.convert_index(i)]
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
/// If a loop is detected returns `Ok(loop)` otherwise returns a `Vec` with all computed values.
///
/// `compute_next_state` must be pure, i.e. the returned value depends only of the input value,
/// otherwise this function may returns false positive.
pub fn detect_loop<T>(
    cycle_count: usize,
    init_state: T,
    mut compute_next_state: impl FnMut(&T) -> T,
) -> Result<Loop<T>, Vec<T>>
where
    T: Eq,
{
    let mut states = vec![init_state];

    for _ in 0..cycle_count {
        // SAFETY: `states` contains at least 1 element.
        let current_state = unsafe { states.last().unwrap_unchecked() };
        let next_state = compute_next_state(current_state);

        let loop_start = states
            .iter()
            .enumerate()
            .find_map(|(i, x)| (x == &next_state).then_some(i));

        if let Some(loop_start) = loop_start {
            return Ok(Loop {
                values: states.into_boxed_slice(),
                loop_start,
            });
        } else {
            states.push(next_state);
        }
    }

    Err(states)
}

/// Computes the value of `init_state` after repeatedly applying `compute_next_state`
/// for `cycle_count` times, using a loop detection strategy.
pub fn compute_last_state<T>(
    cycle_count: usize,
    init_state: T,
    compute_next_state: impl FnMut(&T) -> T,
) -> T
where
    T: Eq,
{
    match detect_loop(cycle_count, init_state, compute_next_state) {
        Ok(the_loop) => the_loop.get_into(cycle_count),
        Err(mut states) => states.pop().unwrap(),
    }
}

/* -------------------------------------------------------------------------- */
