use rust_quadratic_solver::quadratic::{solve_quadratic, QuadraticRoots};
use structopt::StructOpt;

// ------------------------------------------------------------------------------

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
}

// ------------------------------------------------------------------------------

fn main() {
    let opt = Opt::from_args();

    match solve_quadratic(opt.a, opt.b, opt.c) {
        QuadraticRoots::Real((root1, root2)) => {
            println!("Real Quadratic Roots:");
            println!("Root1: {:.4}", root1);
            println!("Root2: {:.4}", root2);
        }
        QuadraticRoots::NotReal((root1, root2)) => {
            println!("Complex Quadratic Roots:");
        }
    };

    // TODO: Print results to a file
}
