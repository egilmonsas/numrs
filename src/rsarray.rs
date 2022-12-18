#[derive(Debug)]
pub struct rsarray {
    pub items: Vec<f64>,
}

impl rsarray {
    pub fn sum(&self) -> f64 {
        self.items.iter().sum()
    }

    pub fn powi(&self, exponent: i32) -> Self {
        let items: Vec<f64> = self.items.iter().map(|x| x.powi(exponent)).collect();
        Self { items }
    }

    pub fn powf(&self, exponent: f64) -> Self {
        let items: Vec<f64> = self.items.iter().map(|x| x.powf(exponent)).collect();
        Self { items }
    }

    pub fn rms(&self) -> f64 {
        self.powi(2).mean().sqrt()
    }

    pub fn mean(&self) -> f64 {
        self.sum() / (self.items.len() as f64)
    }

    fn sort(&mut self) -> &mut Self {
        self.items.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self
    }

    fn sorted(&self) -> Self {
        let mut citems = self.items.clone();
        citems.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Self { items: citems }
    }

    fn clamp(&mut self, min: f64, max: f64) -> &mut Self {
        self.items = self.items.iter().map(|x| x.clamp(min, max)).collect();
        self
    }

    fn clamped(&self, min: f64, max: f64) -> Self {
        Self {
            items: self.items.iter().map(|x| x.clamp(min, max)).collect(),
        }
    }

    pub fn min(&self) -> f64 {
        let mut min = self.items[0];
        self.items.iter().for_each(|x| {
            if x < &min {
                min = *x
            }
        });
        min
    }

    pub fn max(&self) -> f64 {
        let mut max = self.items[0];
        self.items.iter().for_each(|x| {
            if x > &max {
                max = *x
            }
        });
        max
    }

    pub fn percentile(&self, prc: f64) -> f64 {
        let n = self.items.len() as f64;
        self.sorted().items[(prc * n) as usize]
    }

    pub fn var(&self) -> f64 {
        let mu = self.mean();
        self.items
            .iter()
            .map(|x: &f64| (mu - x).powi(2))
            .sum::<f64>()
            / (self.items.len() as f64)
    }

    pub fn std(&self) -> f64 {
        self.var().sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TOLERANCE: f64 = 0.0001;

    #[test]
    fn test_sum() {
        let items = vec![0.7, 0.2, 0.1];
        let a = rsarray { items };
        assert!((a.sum() - 1.0).abs() < TOLERANCE);
    }

    #[test]
    fn test_mean() {
        let items = vec![0.7, 0.2, 0.1];
        let a = rsarray { items };
        assert!((a.mean() - 0.33333).abs() < TOLERANCE);
    }

    #[test]
    fn test_sorted() {
        let items = vec![0.7, 0.2, 0.1];
        let sorted = vec![0.1, 0.2, 0.7];
        let a = rsarray { items };
        assert_eq!(a.sorted().items, sorted);
    }

    #[test]
    fn test_min() {
        let items = vec![0.7, 0.2, 0.1];
        let a = rsarray { items };
        assert!((a.min() - 0.1).abs() < TOLERANCE);
    }

    #[test]
    fn test_max() {
        let items = vec![0.7, 0.2, 0.1];
        let a = rsarray { items };
        assert!((a.max() - 0.7).abs() < TOLERANCE);
    }

    #[test]
    fn test_power_sum() {
        let items = vec![0.7, 0.2, 0.1];
        let a = rsarray { items };
        let expexted_result = 0.49 + 0.04 + 0.01;
        assert!((a.powi(2).sum() - expexted_result).abs() < TOLERANCE);
        assert!((a.powf(2.0).sum() - expexted_result).abs() < TOLERANCE);
    }

    #[test]
    fn test_root_mean_square() {
        let items = vec![0.7, 0.2, 0.1];
        let a = rsarray { items };
        assert!((a.rms() - 0.424264).abs() < TOLERANCE);
    }

    #[test]
    fn test_clamp() {
        let items = vec![0.7, 0.2, 0.1];
        let mut a = rsarray { items };
        assert_eq!(a.clamp(0.2, 0.5).items, vec![0.5, 0.2, 0.2]);
    }

    #[test]
    fn test_clamped() {
        let items = vec![0.7, 0.2, 0.1];
        let mut a = rsarray { items };
        assert_eq!(a.clamped(0.2, 0.5).items, vec![0.5, 0.2, 0.2]);
    }
}
