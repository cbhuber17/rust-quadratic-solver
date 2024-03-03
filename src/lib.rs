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
            let root1 = Complex::new(-1.0 * b / (2.0 * a), (-1.0 * d).sqrt() / (2.0 * a));
            let root2 = Complex::new(-1.0 * b / (2.0 * a), -1.0 * (-1.0 * d).sqrt() / (2.0 * a));
            QuadraticRoots::NotReal((root1, root2))
        } else {
            let root1 = (-1.0 * b + d.sqrt()) / (2.0 * a);
            let root2 = (-1.0 * b - d.sqrt()) / (2.0 * a);
            QuadraticRoots::Real((root1, root2))
        }
    }
}

// ------------------------------------------------------------------------------

pub mod utils {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    use crate::quadratic::QuadraticRoots;

    pub fn create_file(filename: &Path) -> File {
        match filename.extension() {
            Some(ext) => {
                if ext != "csv" {
                    panic!("\nERROR: File extension must be .csv")
                }
            }
            _ => {
                panic!("\nERROR: File extension must be .csv")
            }
        }

        match File::create(&filename) {
            Ok(file) => file,
            Err(err) => panic!(
                "ERROR: Could not create file: {} due to: {}",
                filename.display(),
                err
            ),
        }
    }

    pub fn print_results(root_type: QuadraticRoots) {
        match root_type {
            QuadraticRoots::Real((root1, root2)) => {
                println!(
                    "Real Quadratic Roots:\nRoot1: {:.4}\nRoot2: {:.4}",
                    root1, root2
                )
            }
            QuadraticRoots::NotReal((root1, root2)) => {
                println!(
                    "Complex Quadratic Roots:\nRoot1: {:.4} +{:.4}i\nRoot2: {:.4} {:.4}i",
                    root1.re, root1.im, root2.re, root2.im
                )
            }
        }
    }

    pub fn write_results_to_file(root_type: QuadraticRoots, mut output_file: &File, path: &Path) {
        match root_type {
            QuadraticRoots::Real((root1, root2)) => {
                match output_file.write_all(format!("v1,r,{:.4},{:.4}", root1, root2).as_bytes()) {
                    Ok(_) => print_success_message(path),
                    Err(err) => print_error_message(path, err),
                }
            }
            QuadraticRoots::NotReal((root1, root2)) => {
                match output_file.write_all(format!("v1,c,{:.4},{:.4}", root1, root2).as_bytes()) {
                    Ok(_) => print_success_message(path),
                    Err(err) => print_error_message(path, err),
                }
            }
        }
    }

    fn print_success_message(path: &Path) {
        println!("\n\nSuccessfully wrote results to: {}", path.display());
    }

    fn print_error_message(path: &Path, err: std::io::Error) {
        println!(
            "ERROR: Could not write to file: {} due to: {}",
            path.display(),
            err
        );
    }
}

// ----------- UNIT TESTING ---------------------------------

#[cfg(test)]
mod test {
    use num::Complex;

    // Everything above will be in scope for test
    use super::quadratic::*;

    #[test]
    fn test_real_quadratic() {
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

    #[test]
    fn test_complex_quadratic() {
        assert_eq!(
            solve_quadratic(1.0, 1.0, 1.0),
            QuadraticRoots::NotReal((
                Complex::new(-0.5, f32::sqrt(3.0) / 2.0),
                Complex::new(-0.5, -1.0 * f32::sqrt(3.0) / 2.0)
            ))
        );
    }
}
