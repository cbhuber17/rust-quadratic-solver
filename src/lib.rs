// TODO: Use complex solutions for non-real roots

pub mod quadratic {
    pub fn solve_quadratic(a: f32, b: f32, c: f32) -> (f32, f32) {
        // TODO: Imaginary numbers if d < 0 ?

        let d = b * b - 4.0 * a * c;

        if d < 0.0 {
            println!(
                "\nError:\na: {}\nb: {}\nc: {}\ndoes not have real roots.",
                a, b, c
            );
            panic!()
        }

        let root1: f32 = (-1.0 * b + d.sqrt()) / (2.0 * a);
        let root2: f32 = (-1.0 * b - d.sqrt()) / (2.0 * a);
        (root1, root2)
    }
}

// ----------- UNIT TESTING ---------------------------------

#[cfg(test)]
mod test {
    // Everything above will be in scope for test
    use super::quadratic::*;

    #[test]
    fn test_valid_quadratic() {
        assert_eq!(solve_quadratic(1.0, -7.0, 10.0), (5.0, 2.0));
        assert_eq!(solve_quadratic(1.0, -1.0, -6.0), (3.0, -2.0));
        assert_eq!(solve_quadratic(2.0, 5.0, -3.0), (0.5, -3.0));
        assert_eq!(
            solve_quadratic(-2.0, -3.0, 7.0),
            (
                (3.0 + f32::sqrt(65.0)) / -4.0,
                (3.0 - f32::sqrt(65.0)) / -4.0
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_invalid_quadratic() {
        assert_eq!(solve_quadratic(0.0, -1.0, -6.0), (f32::INFINITY, f32::NAN));
    }
}
