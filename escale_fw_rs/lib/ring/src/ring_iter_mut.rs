use crate::{ring::Ring, ring_state::Behavior};

pub struct RingIterMut<'a, T, const N: usize, B: Behavior> {
    ring: &'a mut Ring<T, N, B>,
    index: usize,
}

impl<'a, T, const N: usize, B: Behavior> RingIterMut<'a, T, N, B> {
    pub fn new(ring: &'a mut Ring<T, N, B>) -> Self {
        Self { ring, index: 0 }
    }
}

impl<'a, T, const N: usize, B: Behavior> Iterator for RingIterMut<'a, T, N, B> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.ring.count() {
            let value = &mut self.ring[self.index];
            self.index += 1;
            Some(unsafe { &mut *(value as *mut _) })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let c = self.ring.count();
        (c, Some(c))
    }

    fn count(self) -> usize {
        self.ring.count()
    }

    fn last(self) -> Option<Self::Item> {
        self.ring.last_mut()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n < self.ring.count() {
            let value = &mut self.ring[n];
            Some(unsafe { &mut *(value as *mut _) })
        } else {
            None
        }
    }
}

impl<'a, T, const N: usize, B: Behavior> IntoIterator for &'a mut Ring<T, N, B> {
    type Item = &'a mut T;
    type IntoIter = RingIterMut<'a, T, N, B>;

    fn into_iter(self) -> Self::IntoIter {
        RingIterMut::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ring_state::{Error, Saturating};

    #[test]
    fn iter() -> Result<(), Error> {
        let mut r: Ring<u32, 2, Saturating> = Default::default();

        r.push(0)?;
        r.pop_front()?;

        r.push(1)?;
        r.push(2)?;

        let mut i = (&mut r).into_iter();

        let x = i.next().unwrap();
        *x += 10;
        assert_eq!(*x, 11);

        let x = i.next().unwrap();
        *x += 10;
        assert_eq!(*x, 12);

        assert_eq!(i.next(), None);

        Ok(())
    }
}
