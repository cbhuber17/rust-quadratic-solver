use rust_quadratic_solver::quadratic::{solve_quadratic, QuadraticRoots};
use rust_quadratic_solver::utils::{create_file, print_results, write_results_to_file};
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

// ------------------------------------------------------------------------------

/// Input parameters for the program.
#[derive(Debug, StructOpt)]
#[structopt(
    long_about = "Quadratic Equation Solver ax^2 + bx + c\nExample: rust-quadratic-solver.exe -- -a 1 -b -2 -c -3 -f output.csv"
)]
#[structopt(author = "Created by Colin Huber")]
#[structopt(global_setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
struct Opt {
    #[structopt(short = "a", required = true)]
    a: f32,

    #[structopt(short = "b", required = true)]
    b: f32,

    #[structopt(short = "c", required = true)]
    c: f32,

    /// Output file
    #[structopt(short = "f", long = "file", required = true)]
    f: PathBuf,
}

// ------------------------------------------------------------------------------

/// Main entry point to the program.
fn main() {
    let opt = Opt::from_args();
    if opt.a == 0.0 {
        panic!("ERROR: Parameter 'a' cannot be zero.");
    }
    let path = Path::new(&opt.f);

    let output_file = create_file(path);

    match solve_quadratic(opt.a, opt.b, opt.c) {
        QuadraticRoots::Real((root1, root2)) => {
            print_results(QuadraticRoots::Real((root1, root2)));

            write_results_to_file(QuadraticRoots::Real((root1, root2)), &output_file, path)
        }
        QuadraticRoots::NotReal((root1, root2)) => {
            print_results(QuadraticRoots::NotReal((root1, root2)));

            write_results_to_file(QuadraticRoots::NotReal((root1, root2)), &output_file, path)
        }
    };
}
