use num_traits::{float::FloatCore, Num, NumCast};

// pub fn mean<N, R>(values: &[N]) -> Option<R>
pub fn mean<Iter, N, R>(iter: Iter) -> Option<R>
where
    N: Num + NumCast,
    R: FloatCore,
    Iter: Iterator<Item = N>,
{
    const E_SUM_SIZE: &str = "the sum should fit into the same numeric type as the items";
    const E_LEN_SIZE: &str = "the length should fit into the same numeric type as the items";

    let (count, sum) = iter.fold((0usize, N::zero()), |state, x| {
        let (count, sum) = state;
        (count + 1, sum + x)
    });
    if count > 0 {
        Some(R::from(sum).expect(E_SUM_SIZE) / R::from(count).expect(E_LEN_SIZE))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_works() {
        let xs = [1, 2, 3, 4];
        assert_eq!(mean(xs.iter().copied()), Some(2.5));
    }

    #[test]
    fn mean_of_empty_is_none() {
        let xs: [i32; 0] = [];
        assert_eq!(mean::<_, _, f32>(xs.iter().copied()), None);
    }
}
