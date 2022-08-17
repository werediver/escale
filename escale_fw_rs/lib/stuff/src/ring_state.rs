use core::marker::PhantomData;

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
pub struct Saturating;

impl Behavior for Saturating {}

/// `RingState<_, Wrapping>` suggests overwriting existing elements when
/// the capacity is exhausted. Push operations never fail.
pub struct Wrapping;

impl Behavior for Wrapping {}

pub trait AnyRingState {
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
    fn head(&self) -> usize;
    fn tail(&self) -> usize;
    fn dec_tail(&mut self) -> Result<(), Error>;
    fn inc_head(&mut self) -> Result<(), Error>;
    fn inc_tail(&mut self) -> Result<(), Error>;
    fn dec_head(&mut self) -> Result<(), Error>;
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
    pub fn is_empty(&self) -> bool {
        self.head == self.tail && !self.is_full
    }

    pub fn is_full(&self) -> bool {
        self.head == self.tail && self.is_full
    }

    pub fn head(&self) -> usize {
        self.head
    }

    pub fn tail(&self) -> usize {
        self.tail
    }

    pub fn dec_tail(&mut self) -> Result<(), Error> {
        if !self.is_empty() {
            self.tail = Self::dec_index(self.tail);
            self.is_full = false;
            Ok(())
        } else {
            Err(Error::Empty)
        }
    }

    pub fn inc_head(&mut self) -> Result<(), Error> {
        if !self.is_empty() {
            self.head = Self::inc_index(self.head);
            self.is_full = false;
            Ok(())
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
    pub fn inc_tail(&mut self) -> Result<(), Error> {
        if !self.is_full() {
            self.tail = Self::inc_index(self.tail);
            self.is_full = self.head == self.tail;
            Ok(())
        } else {
            Err(Error::Full)
        }
    }

    pub fn dec_head(&mut self) -> Result<(), Error> {
        if !self.is_full() {
            self.head = Self::dec_index(self.head);
            self.is_full = self.head == self.tail;
            Ok(())
        } else {
            Err(Error::Full)
        }
    }
}

impl<const N: usize> RingState<N, Wrapping> {
    pub fn inc_tail(&mut self) {
        let is_full = self.is_full();
        self.tail = Self::inc_index(self.tail);
        if !is_full {
            self.is_full = self.head == self.tail;
        } else {
            self.head = self.tail;
        }
    }

    pub fn dec_head(&mut self) {
        let is_full = self.is_full();
        self.head = Self::dec_index(self.head);
        if !is_full {
            self.is_full = self.head == self.tail;
        } else {
            self.tail = self.head;
        }
    }
}

impl<const N: usize> AnyRingState for RingState<N, Saturating> {
    fn is_empty(&self) -> bool {
        RingState::is_empty(self)
    }

    fn is_full(&self) -> bool {
        RingState::is_full(self)
    }

    fn head(&self) -> usize {
        RingState::head(self)
    }

    fn tail(&self) -> usize {
        RingState::tail(self)
    }

    fn dec_tail(&mut self) -> Result<(), Error> {
        RingState::dec_tail(self)
    }

    fn inc_head(&mut self) -> Result<(), Error> {
        RingState::inc_head(self)
    }

    fn inc_tail(&mut self) -> Result<(), Error> {
        RingState::<N, Saturating>::inc_tail(self)
    }

    fn dec_head(&mut self) -> Result<(), Error> {
        RingState::<N, Saturating>::dec_head(self)
    }
}

impl<const N: usize> AnyRingState for RingState<N, Wrapping> {
    fn is_empty(&self) -> bool {
        RingState::is_empty(self)
    }

    fn is_full(&self) -> bool {
        RingState::is_full(self)
    }

    fn head(&self) -> usize {
        RingState::head(self)
    }

    fn tail(&self) -> usize {
        RingState::tail(self)
    }

    fn dec_tail(&mut self) -> Result<(), Error> {
        RingState::dec_tail(self)
    }

    fn inc_head(&mut self) -> Result<(), Error> {
        RingState::inc_head(self)
    }

    fn inc_tail(&mut self) -> Result<(), Error> {
        RingState::<N, Wrapping>::inc_tail(self);
        Ok(())
    }

    fn dec_head(&mut self) -> Result<(), Error> {
        RingState::<N, Wrapping>::dec_head(self);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;

    macro_rules! for_all_behaviors {
        ($func:ident) => {
            paste! {
                #[test]
                fn [<$func _saturating>]() -> Result<(), Error> {
                    $func::<Saturating>()?;
                    Ok(())
                }

                #[test]
                fn [<$func _wrapping>]() -> Result<(), Error> {
                    $func::<Wrapping>()?;
                    Ok(())
                }
            }
        };
    }

    for_all_behaviors!(zero_capacity_default_is_full);

    fn zero_capacity_default_is_full<B: Behavior>() -> Result<(), Error> {
        let i: RingState<0, B> = Default::default();
        assert_eq!(i.head(), 0);
        assert_eq!(i.tail(), 0);
        assert!(!i.is_empty());
        assert!(i.is_full());
        Ok(())
    }

    for_all_behaviors!(default_is_empty);

    fn default_is_empty<B: Behavior>() -> Result<(), Error> {
        let i: RingState<2, B> = Default::default();
        assert_eq!(i.head(), 0);
        assert_eq!(i.tail(), 0);
        assert!(i.is_empty());
        assert!(!i.is_full());
        Ok(())
    }

    for_all_behaviors!(tail_basics);

    fn tail_basics<B: Behavior>() -> Result<(), Error>
    where
        RingState<2, B>: AnyRingState,
    {
        let mut i: RingState<2, B> = Default::default();
        i.inc_tail()?;
        assert_eq!(i.tail(), 1);
        i.inc_tail()?;
        assert_eq!(i.tail(), 0);

        assert!(!i.is_empty());
        assert!(i.is_full());

        i.dec_tail()?;
        assert_eq!(i.tail(), 1);
        i.dec_tail()?;
        assert_eq!(i.tail(), 0);

        assert!(i.is_empty());
        assert!(!i.is_full());

        Ok(())
    }

    for_all_behaviors!(head_basics);

    fn head_basics<B: Behavior>() -> Result<(), Error>
    where
        RingState<2, B>: AnyRingState,
    {
        let mut i: RingState<2, B> = Default::default();
        i.dec_head()?;
        assert_eq!(i.head(), 1);
        i.dec_head()?;
        assert_eq!(i.head(), 0);

        assert!(!i.is_empty());
        assert!(i.is_full());

        i.inc_head()?;
        assert_eq!(i.head(), 1);
        i.inc_head()?;
        assert_eq!(i.head(), 0);

        assert!(i.is_empty());
        assert!(!i.is_full());

        Ok(())
    }

    #[test]
    fn errors_saturating() -> Result<(), Error> {
        let mut i: RingState<2, Saturating> = Default::default();
        assert_eq!(i.dec_tail(), Err(Error::Empty));
        assert_eq!(i.inc_head(), Err(Error::Empty));

        i.inc_tail()?;
        i.inc_tail()?;

        assert_eq!(i.inc_tail(), Err(Error::Full));
        assert_eq!(i.dec_head(), Err(Error::Full));

        Ok(())
    }

    #[test]
    fn errors_wrapping() {
        let mut i: RingState<2, Wrapping> = Default::default();
        assert_eq!(i.dec_tail(), Err(Error::Empty));
        assert_eq!(i.inc_head(), Err(Error::Empty));

        i.inc_tail();
        i.inc_tail();

        i.inc_tail();
        assert_eq!(i.tail(), 1);
        assert_eq!(i.head(), 1);
        i.dec_head();
        assert_eq!(i.head(), 0);
        assert_eq!(i.tail(), 0);
    }
}
