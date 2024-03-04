/// Module containing functionality to solve quadratic equations,
/// whether they have real or complex roots.
pub mod quadratic {
    use num::complex::Complex;

    /// Represents the roots of a quadratic equation.
    #[derive(Debug, PartialEq)]
    pub enum QuadraticRoots {
        Real((f32, f32)),
        NotReal((Complex<f32>, Complex<f32>)),
    }

    /// Solves a quadratic equation and returns its roots.
    ///
    /// # Arguments
    ///
    /// * `a` - The coefficient of the quadratic term.
    /// * `b` - The coefficient of the linear term.
    /// * `c` - The constant term.
    ///
    /// # Returns
    ///
    /// * If the roots are real, returns `QuadraticRoots::Real((root1, root2))`.
    /// * If the roots are complex, returns `QuadraticRoots::NotReal((root1, root2))`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_quadratic_solver::quadratic::{QuadraticRoots, solve_quadratic};
    /// use num::complex::Complex;
    ///
    /// let roots = solve_quadratic(1.0, 0.0, -1.0);
    /// assert_eq!(roots, QuadraticRoots::Real((1.0, -1.0)));
    ///
    /// let roots = solve_quadratic(1.0, 0.0, 1.0);
    /// assert_eq!(roots, QuadraticRoots::NotReal((Complex::new(0.0, 1.0), Complex::new(0.0, -1.0))));
    /// ```
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

/// A utility module to support I/O operations of the program.
/// Contains functions to create an output file, print the roots to the
/// console and file, and any success/error messages.
pub mod utils {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    use crate::quadratic::QuadraticRoots;

    /// Creates a new CSV file with the specified filename.
    ///
    /// # Arguments
    ///
    /// * `filename` - A reference to the path where the new file should be created.
    ///
    /// # Returns
    ///
    /// Returns a `File` object representing the newly created CSV file.
    ///
    /// # Panics
    ///
    /// Panics if the file extension is not `.csv` or if there was an error creating the file.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_quadratic_solver::utils::create_file;
    /// use std::path::Path;
    ///
    /// let filename = Path::new("example.csv");
    /// let file = create_file(&filename);
    /// ```
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

    /// Prints the roots of a quadratic equation to the console.
    ///
    /// # Arguments
    ///
    /// * `root_type` - The type of roots to print, either `QuadraticRoots::Real` or `QuadraticRoots::NotReal`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_quadratic_solver::utils::print_results;
    /// use rust_quadratic_solver::quadratic::QuadraticRoots;
    /// use num::complex::Complex;
    ///
    /// let roots_real = QuadraticRoots::Real((1.0, -1.0));
    /// let roots_complex = QuadraticRoots::NotReal((Complex::new(0.0, 1.0), Complex::new(0.0, -1.0)));
    ///
    /// print_results(roots_real);
    /// print_results(roots_complex);
    /// ```
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

    /// Writes the roots of a quadratic equation to a file.
    ///
    /// # Arguments
    ///
    /// * `root_type` - The type of roots to write, either `QuadraticRoots::Real` or `QuadraticRoots::NotReal`.
    /// * `output_file` - A mutable reference to the file where the roots should be written.
    /// * `path` - The path to the file where the roots should be written.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_quadratic_solver::utils::write_results_to_file;
    /// use rust_quadratic_solver::quadratic::QuadraticRoots;
    /// use std::fs::File;
    /// use std::path::Path;
    ///
    /// let roots_real = QuadraticRoots::Real((1.0, -1.0));
    /// let mut file = File::create("output.txt").unwrap();
    /// let path = Path::new("output.txt");
    ///
    /// write_results_to_file(roots_real, &mut file, &path);
    /// ```
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

    /// Prints a success message indicating that the results were written to a file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file where the results were written.
    fn print_success_message(path: &Path) {
        println!("\n\nSuccessfully wrote results to: {}", path.display());
    }

    /// Prints an error message indicating that there was an error writing to a file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file where the error occurred.
    /// * `err` - The error that occurred while writing to the file.
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
