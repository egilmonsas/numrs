use std::{io::Error, io::ErrorKind};

use crate::rsarray::rsarray;

#[derive(Debug)]
struct Lerp {
    known_xs: rsarray,
    known_ys: rsarray,
}

impl Lerp {
    fn new(known_xs: &rsarray, known_ys: &rsarray) -> Result<Self, Error> {
        if known_xs.len() != known_ys.len() {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Arraylengths must be equal",
            ))
        } else {
            // Fucking ugly (and slow probably) way to sort both vecs by the known x values
            let mut tup_vec = known_xs
                .items
                .iter()
                .zip(known_ys.items.iter())
                .collect::<Vec<(&f64, &f64)>>();

            tup_vec.sort_by(|(&x1, _), (x2, _)| x1.partial_cmp(x2).unwrap());

            let mut veca = Vec::new();
            let mut vecb = Vec::new();

            tup_vec.iter().for_each(|&(a, b)| {
                veca.push(*a);
                vecb.push(*b);
            });

            Ok(Self {
                known_xs: rsarray { items: veca },
                known_ys: rsarray { items: vecb },
            })
        }
    }

    fn eval(&self, x: f64) -> f64 {
        if x <= self.known_xs.min() {
            /* x below range */
            *self.known_ys.items.first().unwrap()
        } else if x >= self.known_xs.max() {
            /* x above range */
            *self.known_ys.items.last().unwrap()
        } else {
            /* x inside range */

            let (idx, _) = self.known_xs.find(&x);

            // index into and read values
            let x_prev = self.known_xs[idx as usize];
            let x_next = self.known_xs[idx as usize + 1];
            let y_prev = self.known_ys[idx as usize];
            let y_next = self.known_ys[idx as usize + 1];

            // compute interpolation distance
            let progression_percent = {
                let width = x_next - x_prev;
                let progression = x - x_prev;
                progression / width
            };

            y_prev + (y_next - y_prev) * progression_percent
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_zeq, utl::ZEq};

    use super::*;

    #[test]
    fn test1() {
        let known_xs = rsarray {
            items: vec![0.0, 1.0],
        };
        let known_ys = rsarray {
            items: vec![0.0, 1.0],
        };

        let lrp = Lerp::new(&known_xs, &known_ys).unwrap();

        assert_zeq!(lrp.eval(0.5), 0.5);
        assert_zeq!(lrp.eval(0.7), 0.7);
        assert_zeq!(lrp.eval(0.2), 0.2);
        assert_zeq!(lrp.eval(0.0), 0.0);
        assert_zeq!(lrp.eval(-2.0), 0.0);
        assert_zeq!(lrp.eval(2.0), 1.0);
    }
    #[test]
    fn test2() {
        let known_xs = rsarray {
            items: vec![0.0, 2.0, 1.0],
        };
        let known_ys = rsarray {
            items: vec![0.0, 2.0, 3.0],
        };

        let lrp = Lerp::new(&known_xs, &known_ys).unwrap();

        assert_zeq!(lrp.eval(2.0), 2.0);
    }
}
