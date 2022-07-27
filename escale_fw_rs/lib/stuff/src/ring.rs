use core::iter::Chain;

pub struct Ring<T, const N: usize> {
    pub data: [T; N],
    end: usize,
}

impl<T: Copy + Default, const N: usize> Default for Ring<T, N> {
    fn default() -> Self {
        Self::new([T::default(); N])
    }
}

impl<T, const N: usize> Ring<T, N> {
    pub fn new(data: [T; N]) -> Self {
        Self { data, end: 0 }
    }

    pub fn push(&mut self, value: T) {
        self.data[self.end] = value;
        self.end = self.end.wrapping_add(1) % N;
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

impl<'a, T, const N: usize> IntoIterator for &'a Ring<T, N> {
    type Item = &'a T;
    type IntoIter = Chain<<&'a [T] as IntoIterator>::IntoIter, <&'a [T] as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut Ring<T, N> {
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
        let ring = Ring::<i32, 3>::default();
        assert_eq!(ring.data, [0; 3]);
    }

    #[test]
    fn new_works() {
        let ring = Ring::new([1, 2, 3]);
        assert_eq!(ring.data, [1, 2, 3]);
    }

    #[test]
    fn iter_works() {
        let mut ring = Ring::new([1, 2, 3]);
        ring.push(4);
        ring.push(5);
        ring.push(6);
        ring.push(7);
        assert!(ring.iter().eq([5, 6, 7].iter()));
    }

    #[test]
    fn iter_mut_works() {
        let mut ring = Ring::new([1, 2, 3]);
        for x in ring.iter_mut() {
            *x += 1;
        }
        assert!(ring.iter_mut().eq([2, 3, 4].iter()));
    }

    #[test]
    fn into_iter_works() {
        let mut ring = Ring::new([1, 2, 3]);
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
        let mut ring = Ring::new([1, 2, 3]);
        for x in &mut ring {
            *x += 1;
        }
        assert!(ring.iter_mut().eq([2, 3, 4].iter()));
    }
}
