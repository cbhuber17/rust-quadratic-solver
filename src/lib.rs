// TODO: Use complex solutions for non-real roots

pub mod quadratic {
    use num::complex::Complex;

    #[derive(Debug, PartialEq)]
    pub enum QuadraticRoots {
        Real((f32, f32)),
        NotReal((Complex<f32>, Complex<f32>)),
    }

    pub fn solve_quadratic(a: f32, b: f32, c: f32) -> QuadraticRoots {
        // Discriminant
        let d = b * b - 4.0 * a * c;

        if d < 0.0 {
            let root1 = Complex::new(-1.0 * b / (2.0 * a), (-1.0 * d).sqrt());
            let root2 = Complex::new(-1.0 * b / (2.0 * a), -1.0 * (-1.0 * d).sqrt());
            QuadraticRoots::NotReal((root1, root2))
        } else {
            let root1 = (-1.0 * b + d.sqrt()) / (2.0 * a);
            let root2 = (-1.0 * b - d.sqrt()) / (2.0 * a);
            QuadraticRoots::Real((root1, root2))
        }
    }
}

// ----------- UNIT TESTING ---------------------------------

#[cfg(test)]
mod test {
    // Everything above will be in scope for test
    use super::quadratic::*;

    #[test]
    fn test_valid_quadratic() {
        assert_eq!(
            solve_quadratic(1.0, -7.0, 10.0),
            QuadraticRoots::Real((5.0, 2.0))
        );

        assert_eq!(
            solve_quadratic(1.0, -1.0, -6.0),
            QuadraticRoots::Real((3.0, -2.0))
        );

        assert_eq!(
            solve_quadratic(2.0, 5.0, -3.0),
            QuadraticRoots::Real((0.5, -3.0))
        );

        assert_eq!(
            solve_quadratic(-2.0, -3.0, 7.0),
            QuadraticRoots::Real((
                (3.0 + f32::sqrt(65.0)) / -4.0,
                (3.0 - f32::sqrt(65.0)) / -4.0
            ))
        );
    }

    // TODO: Test complex results
}
