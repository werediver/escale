use core::iter::Chain;

#[derive(Debug)]
pub struct SimpleRing<T, const N: usize>
where
    // Require N ≥ 1
    [(); N - 1]:,
{
    pub data: [T; N],
    end: usize,
    is_filled: bool,
}

impl<T: Copy + Default, const N: usize> Default for SimpleRing<T, N>
where
    [(); N - 1]:,
{
    fn default() -> Self {
        Self {
            data: [T::default(); N],
            end: 0,
            is_filled: false,
        }
    }
}

impl<T, const N: usize> SimpleRing<T, N>
where
    [(); N - 1]:,
{
    pub fn is_filled(&self) -> bool {
        self.is_filled
    }

    pub fn reset(&mut self, value: T)
    where
        T: Clone,
    {
        self.data.fill(value);
        self.is_filled = false;
    }

    #[inline]
    fn advance_end(&mut self) {
        let new_end = self.end.wrapping_add(1) % N;
        if !self.is_filled && new_end <= self.end {
            self.is_filled = true;
        }
        self.end = new_end;
    }

    pub fn push(&mut self, value: T) {
        self.data[self.end] = value;
        self.advance_end();
    }

    pub fn iter(
        &self,
    ) -> Chain<<&'_ [T] as IntoIterator>::IntoIter, <&'_ [T] as IntoIterator>::IntoIter> {
        let (left, right) = self.data.split_at(self.end);
        right.iter().chain(left.iter())
    }

    pub fn iter_mut(
        &mut self,
    ) -> Chain<<&'_ mut [T] as IntoIterator>::IntoIter, <&'_ mut [T] as IntoIterator>::IntoIter>
    {
        let (left, right) = self.data.split_at_mut(self.end);
        right.iter_mut().chain(left.iter_mut())
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a SimpleRing<T, N>
where
    [(); N - 1]:,
{
    type Item = &'a T;
    type IntoIter = Chain<<&'a [T] as IntoIterator>::IntoIter, <&'a [T] as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut SimpleRing<T, N>
where
    [(); N - 1]:,
{
    type Item = &'a mut T;
    type IntoIter =
        Chain<<&'a mut [T] as IntoIterator>::IntoIter, <&'a mut [T] as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_works() {
        let ring = SimpleRing::<i32, 3>::default();
        assert_eq!(ring.data, [0; 3]);
    }

    #[test]
    fn new_ring_is_not_filled() {
        assert_eq!(SimpleRing::<i32, 3>::default().is_filled(), false);
    }

    #[test]
    fn underfilled_ring_is_not_filled() {
        let mut ring = SimpleRing::<i32, 3>::default();
        ring.push(1);
        ring.push(2);
        assert_eq!(ring.is_filled(), false);
    }

    #[test]
    fn filled_ring_is_filled() {
        let mut ring = SimpleRing::<i32, 3>::default();
        ring.push(1);
        ring.push(2);
        ring.push(3);
        assert_eq!(ring.is_filled(), true);
    }

    #[test]
    fn overfilled_ring_is_filled() {
        let mut ring = SimpleRing::<i32, 3>::default();
        ring.push(1);
        ring.push(2);
        ring.push(3);
        ring.push(4);
        assert_eq!(ring.is_filled(), true);
    }

    #[test]
    fn iter_works() {
        let mut ring = SimpleRing::<i32, 3>::default();
        ring.push(4);
        ring.push(5);
        ring.push(6);
        ring.push(7);
        assert!(ring.iter().eq([5, 6, 7].iter()));
    }

    #[test]
    fn iter_mut_works() {
        let mut ring = SimpleRing::<i32, 3>::default();
        ring.push(4);
        ring.push(5);
        ring.push(6);
        ring.push(7);
        for x in ring.iter_mut() {
            *x += 1;
        }
        assert!(ring.iter_mut().eq([6, 7, 8].iter()));
    }

    #[test]
    fn into_iter_works() {
        let mut ring = SimpleRing::<i32, 3>::default();
        ring.push(4);
        ring.push(5);
        ring.push(6);
        ring.push(7);
        let mut y = 5;
        for &x in &ring {
            assert_eq!(x, y);
            y += 1;
        }
    }

    #[test]
    fn into_iter_mut_works() {
        let mut ring = SimpleRing::<i32, 3>::default();
        ring.push(4);
        ring.push(5);
        ring.push(6);
        ring.push(7);
        for x in &mut ring {
            *x += 1;
        }
        assert!(ring.iter_mut().eq([6, 7, 8].iter()));
    }
}
