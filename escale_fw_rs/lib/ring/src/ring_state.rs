use core::marker::PhantomData;

#[derive(Debug)]
pub struct RingState<const N: usize, B: Behavior> {
    /// Also known as the read index.
    head: usize,
    /// Also known as the write index.
    tail: usize,
    /// The flag is only relevant when the head and tail indices match.
    is_full: bool,
    mode: PhantomData<*const B>,
}

pub trait Behavior {}

/// `RingState<_, Saturating>` doesn't suggest overwriting existing elements and
/// indicates an error when the capacity is insufficient for a push operation.
#[derive(Debug)]
pub struct Saturating;

impl Behavior for Saturating {}

/// `RingState<_, Overwriting>` suggests overwriting existing elements when
/// the capacity is exhausted. Push operations never fail.
#[derive(Debug)]
pub struct Overwriting;

impl Behavior for Overwriting {}

pub trait AnyRingState {
    fn capacity() -> usize;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
    fn count(&self) -> usize;
    fn head(&self) -> usize;
    fn tail(&self) -> usize;
    /// Returns the index to pop the value from, or an error,
    /// if the "empty" condition is met.
    #[must_use = "Use the returned index to remove (pop) a value from the buffer"]
    fn will_pop_back(&mut self) -> Result<usize, Error>;
    /// Returns the index to pop the value from, or an error,
    /// if the "empty" condition is met.
    #[must_use = "Use the returned index to remove (pop) a value from the buffer"]
    fn will_pop_front(&mut self) -> Result<usize, Error>;
    #[must_use = "Check the returned value to finalize (drop) or utilize the value that is going to be overwritten"]
    fn will_push_back(&mut self) -> Result<(usize, Push), Error>;
    #[must_use = "Check the returned value to finalize (drop) or utilize the value that is going to be overwritten"]
    fn will_push_front(&mut self) -> Result<(usize, Push), Error>;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Push {
    WithinCapacity,
    Overwriting,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Error {
    Full,
    Empty,
}

impl<const N: usize, B: Behavior> Default for RingState<N, B> {
    fn default() -> Self {
        Self {
            head: 0,
            tail: 0,
            is_full: N == 0,
            mode: PhantomData,
        }
    }
}

impl<const N: usize, B: Behavior> RingState<N, B> {
    #[cfg(test)]
    fn new_with(head: usize, tail: usize, is_full: bool) -> Self {
        assert!(head < N);
        assert!(tail < N);
        Self {
            head,
            tail,
            is_full,
            mode: PhantomData,
        }
    }

    pub const fn capacity() -> usize {
        N
    }

    pub fn is_empty(&self) -> bool {
        self.head == self.tail && !self.is_full
    }

    pub fn is_full(&self) -> bool {
        self.head == self.tail && self.is_full
    }

    pub fn count(&self) -> usize {
        let head = self.head;
        let tail = self.tail;
        if tail >= head && !self.is_full() {
            tail - head
        } else {
            tail + (N - head)
        }
    }

    pub fn head(&self) -> usize {
        self.head
    }

    pub fn tail(&self) -> usize {
        self.tail
    }

    pub fn index(&self, index: usize) -> Option<usize> {
        if index < self.count() {
            let i = self.head + index;
            if i < N {
                Some(i)
            } else {
                Some(i - N)
            }
        } else {
            None
        }
    }

    /// Returns the index to pop the value from, or an error,
    /// if the "empty" condition is met.
    pub fn will_pop_back(&mut self) -> Result<usize, Error> {
        if !self.is_empty() {
            self.tail = Self::dec_index(self.tail);
            self.is_full = false;
            Ok(self.tail)
        } else {
            Err(Error::Empty)
        }
    }

    /// Returns the index to pop the value from, or an error,
    /// if the "empty" condition is met.
    pub fn will_pop_front(&mut self) -> Result<usize, Error> {
        if !self.is_empty() {
            self.head = Self::inc_index(self.head);
            self.is_full = false;
            Ok(self.head)
        } else {
            Err(Error::Empty)
        }
    }

    fn inc_index(i: usize) -> usize {
        debug_assert!(i < usize::MAX, "an index must be less than the capacity");
        let j = i + 1;
        if j < N {
            j
        } else {
            j - N
        }
    }

    fn dec_index(i: usize) -> usize {
        debug_assert!(i < usize::MAX, "an index must be less than the capacity");
        i.checked_sub(1).unwrap_or(N - 1)
    }
}

impl<const N: usize> RingState<N, Saturating> {
    pub fn will_push_back(&mut self) -> Result<usize, Error> {
        if !self.is_full() {
            let original_tail = self.tail;
            self.tail = Self::inc_index(original_tail);
            self.is_full = self.head == self.tail;
            Ok(original_tail)
        } else {
            Err(Error::Full)
        }
    }

    pub fn will_push_front(&mut self) -> Result<usize, Error> {
        if !self.is_full() {
            let original_head = self.head;
            self.head = Self::dec_index(original_head);
            self.is_full = self.head == self.tail;
            Ok(original_head)
        } else {
            Err(Error::Full)
        }
    }
}

impl<const N: usize> RingState<N, Overwriting> {
    pub fn will_push_back(&mut self) -> (usize, Push) {
        let is_full = self.is_full();
        let original_tail = self.tail;
        self.tail = Self::inc_index(original_tail);
        if !is_full {
            self.is_full = self.head == self.tail;
            (original_tail, Push::WithinCapacity)
        } else {
            self.head = self.tail;
            (original_tail, Push::Overwriting)
        }
    }

    pub fn will_push_front(&mut self) -> (usize, Push) {
        let is_full = self.is_full();
        let original_head = self.head;
        self.head = Self::dec_index(original_head);
        if !is_full {
            self.is_full = self.head == self.tail;
            (original_head, Push::WithinCapacity)
        } else {
            self.tail = self.head;
            (original_head, Push::Overwriting)
        }
    }
}

impl<const N: usize> AnyRingState for RingState<N, Saturating> {
    fn capacity() -> usize {
        RingState::<N, Saturating>::capacity()
    }

    fn is_empty(&self) -> bool {
        RingState::is_empty(self)
    }

    fn is_full(&self) -> bool {
        RingState::is_full(self)
    }

    fn count(&self) -> usize {
        RingState::count(self)
    }

    fn head(&self) -> usize {
        RingState::head(self)
    }

    fn tail(&self) -> usize {
        RingState::tail(self)
    }

    fn will_pop_back(&mut self) -> Result<usize, Error> {
        RingState::will_pop_back(self)
    }

    fn will_pop_front(&mut self) -> Result<usize, Error> {
        RingState::will_pop_front(self)
    }

    fn will_push_back(&mut self) -> Result<(usize, Push), Error> {
        let i = RingState::<N, Saturating>::will_push_back(self)?;
        Ok((i, Push::WithinCapacity))
    }

    fn will_push_front(&mut self) -> Result<(usize, Push), Error> {
        let i = RingState::<N, Saturating>::will_push_front(self)?;
        Ok((i, Push::WithinCapacity))
    }
}

impl<const N: usize> AnyRingState for RingState<N, Overwriting> {
    fn capacity() -> usize {
        RingState::<N, Saturating>::capacity()
    }

    fn is_empty(&self) -> bool {
        RingState::is_empty(self)
    }

    fn is_full(&self) -> bool {
        RingState::is_full(self)
    }

    fn count(&self) -> usize {
        RingState::count(self)
    }

    fn head(&self) -> usize {
        RingState::head(self)
    }

    fn tail(&self) -> usize {
        RingState::tail(self)
    }

    fn will_pop_back(&mut self) -> Result<usize, Error> {
        RingState::will_pop_back(self)
    }

    fn will_pop_front(&mut self) -> Result<usize, Error> {
        RingState::will_pop_front(self)
    }

    fn will_push_back(&mut self) -> Result<(usize, Push), Error> {
        Ok(RingState::<N, Overwriting>::will_push_back(self))
    }

    fn will_push_front(&mut self) -> Result<(usize, Push), Error> {
        Ok(RingState::<N, Overwriting>::will_push_front(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;

    macro_rules! test_for_all_behaviors {
        ($func:ident) => {
            paste! {
                #[test]
                fn [<$func _saturating>]() -> Result<(), Error> {
                    $func::<Saturating>()?;
                    Ok(())
                }

                #[test]
                fn [<$func _overwriting>]() -> Result<(), Error> {
                    $func::<Overwriting>()?;
                    Ok(())
                }
            }
        };
    }

    test_for_all_behaviors!(zero_capacity_default_is_full);

    fn zero_capacity_default_is_full<B: Behavior>() -> Result<(), Error> {
        let i: RingState<0, B> = Default::default();
        assert_eq!(i.head(), 0);
        assert_eq!(i.tail(), 0);
        assert_eq!(i.count(), 0);
        assert!(!i.is_empty());
        assert!(i.is_full());
        Ok(())
    }

    test_for_all_behaviors!(default_is_empty);

    fn default_is_empty<B: Behavior>() -> Result<(), Error> {
        let i: RingState<2, B> = Default::default();
        assert_eq!(i.head(), 0);
        assert_eq!(i.tail(), 0);
        assert_eq!(i.count(), 0);
        assert!(i.is_empty());
        assert!(!i.is_full());
        Ok(())
    }

    test_for_all_behaviors!(extreme_count);

    fn extreme_count<B: Behavior>() -> Result<(), Error> {
        let i: RingState<{ usize::MAX }, B> = RingState::new_with(0, 0, true);
        assert!(!i.is_empty());
        assert!(i.is_full());
        assert_eq!(i.count(), usize::MAX);
        Ok(())
    }

    test_for_all_behaviors!(index_translation);

    fn index_translation<B: Behavior>() -> Result<(), Error> {
        let i: RingState<2, B> = RingState::new_with(1, 1, true);
        assert_eq!(i.index(0), Some(1));
        assert_eq!(i.index(1), Some(0));
        assert_eq!(i.index(2), None);
        Ok(())
    }

    test_for_all_behaviors!(tail_basics);

    fn tail_basics<B: Behavior>() -> Result<(), Error>
    where
        RingState<2, B>: AnyRingState,
    {
        let mut i: RingState<2, B> = Default::default();
        i.will_push_back()?;
        assert_eq!(i.tail(), 1);
        assert_eq!(i.count(), 1);
        i.will_push_back()?;
        assert_eq!(i.tail(), 0);
        assert_eq!(i.count(), 2);

        assert!(!i.is_empty());
        assert!(i.is_full());

        i.will_pop_back()?;
        assert_eq!(i.tail(), 1);
        assert_eq!(i.count(), 1);
        i.will_pop_back()?;
        assert_eq!(i.tail(), 0);
        assert_eq!(i.count(), 0);

        assert!(i.is_empty());
        assert!(!i.is_full());

        Ok(())
    }

    test_for_all_behaviors!(head_basics);

    fn head_basics<B: Behavior>() -> Result<(), Error>
    where
        RingState<2, B>: AnyRingState,
    {
        let mut i: RingState<2, B> = Default::default();
        i.will_push_front()?;
        assert_eq!(i.head(), 1);
        assert_eq!(i.count(), 1);
        i.will_push_front()?;
        assert_eq!(i.head(), 0);
        assert_eq!(i.count(), 2);

        assert!(!i.is_empty());
        assert!(i.is_full());

        i.will_pop_front()?;
        assert_eq!(i.head(), 1);
        assert_eq!(i.count(), 1);
        i.will_pop_front()?;
        assert_eq!(i.head(), 0);
        assert_eq!(i.count(), 0);

        assert!(i.is_empty());
        assert!(!i.is_full());

        Ok(())
    }

    #[test]
    fn errors_saturating() -> Result<(), Error> {
        let mut i: RingState<2, Saturating> = Default::default();
        assert_eq!(i.will_pop_back(), Err(Error::Empty));
        assert_eq!(i.will_pop_front(), Err(Error::Empty));

        i.will_push_back()?;
        i.will_push_back()?;

        assert_eq!(i.will_push_back(), Err(Error::Full));
        assert_eq!(i.will_push_front(), Err(Error::Full));

        Ok(())
    }

    #[test]
    fn errors_wrapping() {
        let mut i: RingState<2, Overwriting> = Default::default();
        assert_eq!(i.will_pop_back(), Err(Error::Empty));
        assert_eq!(i.will_pop_front(), Err(Error::Empty));

        assert_eq!(i.will_push_back(), (0, Push::WithinCapacity));
        assert_eq!(i.will_push_back(), (1, Push::WithinCapacity));

        assert_eq!(i.will_push_back(), (0, Push::Overwriting));
        assert_eq!(i.tail(), 1);
        assert_eq!(i.head(), 1);
        assert_eq!(i.will_push_front(), (1, Push::Overwriting));
        assert_eq!(i.head(), 0);
        assert_eq!(i.tail(), 0);
    }
}
