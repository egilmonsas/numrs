use crate::F;

pub fn mean(array: &Vec<F>) -> F {
    let n = array.len();
    array.iter().sum::<F>() / (n as F)
}

pub fn median(array: &Vec<F>) -> F {
    let mut sort_arr = array.clone();
    sort_arr.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if (sort_arr.len() % 2) == 0 {
        let ind_left = sort_arr.len() / 2 - 1;
        let ind_right = sort_arr.len() / 2;
        (sort_arr[ind_left] + sort_arr[ind_right]) / 2.0
    } else {
        sort_arr[(sort_arr.len() / 2)] as F
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_zeq, utl::ZEq};

    use super::*;

    #[test]
    fn test_median() {
        let a1 = vec![0.0, 2.0, 3.0];
        let a2 = vec![0.0, 2.0, 3.0, 3.0];
        let a3 = vec![0.0, 0.5, 2.0, 3.0, 3.0];
        let a4 = vec![0.0, 2.5, 4.0, 3.0, 1.0];
        assert_zeq!(median(&a1), 2.0);
        assert_zeq!(median(&a2), 2.5);
        assert_zeq!(median(&a3), 2.0);
        assert_zeq!(median(&a4), 2.5);
    }

    #[test]
    fn test_mean() {
        let a1 = vec![0.0, 0.0];
        let a2 = vec![0.0, 2.0];
        let a3 = vec![0.0, 3.0, 2.0];
        let a4 = vec![0.0];
        assert_zeq!(mean(&a1), 0.0);
        assert_zeq!(mean(&a2), 1.0);
        assert_zeq!(mean(&a3), 1.6666667);
        assert_zeq!(mean(&a4), 0.0);
    }
}
