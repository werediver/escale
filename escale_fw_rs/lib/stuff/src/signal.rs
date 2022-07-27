use num_traits::{cast, float::FloatCore, Num, NumCast};

pub fn mean<N, R>(values: &[N]) -> Option<R>
where
    N: Num + NumCast + Copy,
    R: FloatCore,
{
    const E_SUM_SIZE: &str = "the sum should fit into the same numeric type as the items";
    const E_LEN_SIZE: &str = "the length should fit into the same numeric type as the items";

    values
        .iter()
        .copied()
        .reduce(|a, b| a + b)
        .map(|sum| cast::<N, R>(sum).expect(E_SUM_SIZE) / cast(values.len()).expect(E_LEN_SIZE))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_works() {
        let xs = [1, 2, 3, 4];
        assert_eq!(mean(&xs), Some(2.5));
    }

    #[test]
    fn mean_of_empty_is_none() {
        let xs: [i32; 0] = [];
        assert_eq!(mean::<_, f32>(&xs), None);
    }
}
