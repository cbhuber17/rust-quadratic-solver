use rust_quadratic_solver::quadratic::{solve_quadratic, QuadraticRoots};
use structopt::StructOpt;

use std::fs::File;
use std::io::Write;
use std::path::Path;

// ------------------------------------------------------------------------------

// TODO: Add CLI example
#[derive(Debug, StructOpt)]
#[structopt(about = "Quadratic Equation Solver ax^2 + bx + c")]
#[structopt(author = "Created by Colin Huber")]
#[structopt(global_setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
struct Opt {
    #[structopt(short = "a", required = true)]
    a: f32,

    #[structopt(short = "b", required = true)]
    b: f32,

    #[structopt(short = "c", required = true)]
    c: f32,
    // TODO: Add output csv file
}

// ------------------------------------------------------------------------------

fn main() {
    let opt = Opt::from_args();

    let path = Path::new("results.txt");

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(err) => panic!(
            "ERROR: Could not create file: {} due to: {}",
            path.display(),
            err
        ),
    };

    match solve_quadratic(opt.a, opt.b, opt.c) {
        QuadraticRoots::Real((root1, root2)) => {
            let results: String = format!(
                "Real Quadratic Roots:\nRoot1: {:.4}\nRoot2: {:.4}",
                root1, root2
            );

            println!("{}", results);

            // TODO: Write csv format "v1,r,r1,r2"
            match file.write_all(results.as_bytes()) {
                Ok(_) => println!("\n\nSuccessfully wrote results to: {}", path.display()),
                Err(err) => println!(
                    "ERROR: Could not write to file: {} due to: {}",
                    path.display(),
                    err
                ),
            }
        }
        QuadraticRoots::NotReal((root1, root2)) => {
            let results: String = format!(
                "Complex Quadratic Roots:\nRoot1: {:.4} +{:.4}i\nRoot2: {:.4} {:.4}i",
                root1.re, root1.im, root2.re, root2.im
            );

            println!("{}", results);

            match file.write_all(results.as_bytes()) {
                Ok(_) => println!("\n\nSuccessfully wrote results to: {}", path.display()),
                Err(err) => println!(
                    "ERROR: Could not write to file: {} due to: {}",
                    path.display(),
                    err
                ),
            }
        }
    };
}
