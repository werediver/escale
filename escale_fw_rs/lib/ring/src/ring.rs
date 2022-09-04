use core::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

use crate::ring_state::{Behavior, Error, Overwriting, Push, RingState, Saturating};

pub struct Ring<T, const N: usize, B: Behavior> {
    data: [MaybeUninit<T>; N],
    state: RingState<N, B>,
}

impl<T, const N: usize, B: Behavior> Default for Ring<T, N, B> {
    fn default() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            state: Default::default(),
        }
    }
}

impl<T, const N: usize, B: Behavior> Ring<T, N, B> {
    pub const fn capacity() -> usize {
        RingState::<N, B>::capacity()
    }

    pub fn is_empty(&self) -> bool {
        self.state.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.state.is_full()
    }

    pub fn count(&self) -> usize {
        self.state.count()
    }

    pub fn first(&self) -> Option<&T> {
        if !self.is_empty() {
            let i = self.state.head();
            Some(unsafe { self.peek(i) })
        } else {
            None
        }
    }

    pub fn last(&self) -> Option<&T> {
        if !self.is_empty() {
            let i = self.state.tail();
            Some(unsafe { self.peek(i) })
        } else {
            None
        }
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        if !self.is_empty() {
            let i = self.state.tail();
            Some(unsafe { self.peek_mut(i) })
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Result<T, Error> {
        let i = self.state.will_pop_back()?;
        Ok(unsafe { self.take(i) })
    }

    pub fn pop_front(&mut self) -> Result<T, Error> {
        let i = self.state.will_pop_front()?;
        Ok(unsafe { self.take(i) })
    }

    // pub fn retain<F>(&mut self, f: F)
    // where
    //     F: FnMut(&T),
    // {
    //     for
    // }

    unsafe fn take(&mut self, index: usize) -> T {
        self.data[index].assume_init_read()
    }

    unsafe fn peek(&self, index: usize) -> &T {
        self.data[index].assume_init_ref()
    }

    unsafe fn peek_mut(&mut self, index: usize) -> &mut T {
        self.data[index].assume_init_mut()
    }
}

impl<T, const N: usize> Ring<T, N, Saturating> {
    pub fn push(&mut self, value: T) -> Result<(), Error> {
        let i = self.state.will_push_back()?;
        self.data[i] = MaybeUninit::new(value);
        Ok(())
    }

    pub fn push_front(&mut self, value: T) -> Result<(), Error> {
        let i = self.state.will_push_front()?;
        self.data[i] = MaybeUninit::new(value);
        Ok(())
    }
}

impl<T, const N: usize> Ring<T, N, Overwriting> {
    pub fn push(&mut self, value: T) -> Result<Option<T>, Error> {
        let (i, push) = self.state.will_push_back();
        let displaced = match push {
            Push::WithinCapacity => None,
            Push::Overwriting => Some(unsafe { self.take(i) }),
        };
        self.data[i] = MaybeUninit::new(value);
        Ok(displaced)
    }

    pub fn push_front(&mut self, value: T) -> Result<Option<T>, Error> {
        let (i, push) = self.state.will_push_front();
        let displaced = match push {
            Push::WithinCapacity => None,
            Push::Overwriting => Some(unsafe { self.take(i) }),
        };
        self.data[i] = MaybeUninit::new(value);
        Ok(displaced)
    }
}

impl<T, const N: usize, B: Behavior> Index<usize> for Ring<T, N, B> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let i = self
            .state
            .index(index)
            .expect("the index must be within the bounds");
        unsafe { self.peek(i) }
    }
}

impl<T, const N: usize, B: Behavior> IndexMut<usize> for Ring<T, N, B> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let i = self
            .state
            .index(index)
            .expect("the index must be within the bounds");
        unsafe { self.peek_mut(i) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sat_empty_pop_errors() {
        let mut r: Ring<i32, 2, Saturating> = Default::default();
        assert_eq!(r.pop(), Err(Error::Empty));
        assert_eq!(r.pop_front(), Err(Error::Empty));
    }

    #[test]
    fn owr_empty_pop_errors() {
        let mut r: Ring<i32, 2, Overwriting> = Default::default();
        assert_eq!(r.pop(), Err(Error::Empty));
        assert_eq!(r.pop_front(), Err(Error::Empty));
    }

    #[test]
    fn sat_push_pop_back() {
        let mut r: Ring<i32, 2, Saturating> = Default::default();
        assert_eq!(r.push(1), Ok(()));
        assert_eq!(r.push(2), Ok(()));
        assert_eq!(r.push(3), Err(Error::Full));
        assert_eq!(r.pop(), Ok(2));
        assert_eq!(r.pop(), Ok(1));
    }

    #[test]
    fn owr_push_pop_back() {
        let mut r: Ring<i32, 2, Overwriting> = Default::default();
        assert_eq!(r.push(1), Ok(None));
        assert_eq!(r.push(2), Ok(None));
        assert_eq!(r.push(3), Ok(Some(1)));
        assert_eq!(r.pop(), Ok(3));
        assert_eq!(r.pop(), Ok(2));
    }

    #[test]
    fn sat_push_pop_front() {
        let mut r: Ring<i32, 2, Saturating> = Default::default();
        assert_eq!(r.push_front(1), Ok(()));
        assert_eq!(r.push_front(2), Ok(()));
        assert_eq!(r.push_front(3), Err(Error::Full));
        assert_eq!(r.pop(), Ok(2));
        assert_eq!(r.pop(), Ok(1));
    }

    #[test]
    fn owr_push_pop_front() {
        let mut r: Ring<i32, 2, Overwriting> = Default::default();
        assert_eq!(r.push_front(1), Ok(None));
        assert_eq!(r.push_front(2), Ok(None));
        assert_eq!(r.push_front(3), Ok(Some(1)));
        assert_eq!(r.pop(), Ok(3));
        assert_eq!(r.pop(), Ok(2));
    }

    #[test]
    fn sat_index() {
        let mut r: Ring<i32, 2, Saturating> = Default::default();
        assert_eq!(r.push_front(1), Ok(()));
        assert_eq!(r.push_front(2), Ok(()));
        assert_eq!(r[0], 1);
        assert_eq!(r[1], 2);
    }

    #[test]
    fn owr_index() {
        let mut r: Ring<i32, 2, Overwriting> = Default::default();
        assert_eq!(r.push_front(1), Ok(None));
        assert_eq!(r.push_front(2), Ok(None));
        assert_eq!(r[0], 1);
        assert_eq!(r[1], 2);
    }

    #[test]
    fn sat_index_mut() {
        let mut r: Ring<i32, 2, Saturating> = Default::default();
        assert_eq!(r.push_front(1), Ok(()));
        assert_eq!(r.push_front(2), Ok(()));
        r[0] = 10;
        r[1] = 20;
        assert_eq!(r[0], 10);
        assert_eq!(r[1], 20);
    }

    #[test]
    fn owr_index_mut() {
        let mut r: Ring<i32, 2, Overwriting> = Default::default();
        assert_eq!(r.push_front(1), Ok(None));
        assert_eq!(r.push_front(2), Ok(None));
        r[0] = 10;
        r[1] = 20;
        assert_eq!(r[0], 10);
        assert_eq!(r[1], 20);
    }
}
