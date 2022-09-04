use crate::{ring::Ring, ring_state::Behavior};

pub struct RingIter<'a, T, const N: usize, B: Behavior> {
    ring: &'a Ring<T, N, B>,
    index: usize,
}

impl<'a, T, const N: usize, B: Behavior> RingIter<'a, T, N, B> {
    pub fn new(ring: &'a Ring<T, N, B>) -> Self {
        Self { ring, index: 0 }
    }
}

impl<'a, T, const N: usize, B: Behavior> Iterator for RingIter<'a, T, N, B> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.ring.count() {
            let value = &self.ring[self.index];
            self.index += 1;
            Some(value)
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
        self.ring.last()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n < self.ring.count() {
            Some(&self.ring[n])
        } else {
            None
        }
    }
}

impl<'a, T, const N: usize, B: Behavior> IntoIterator for &'a Ring<T, N, B> {
    type Item = &'a T;
    type IntoIter = RingIter<'a, T, N, B>;

    fn into_iter(self) -> Self::IntoIter {
        RingIter::new(self)
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

        let mut i = (&r).into_iter();
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), None);

        Ok(())
    }
}
