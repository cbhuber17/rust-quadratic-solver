use rust_quadratic_solver::quadratic::solve_quadratic;
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

    let (root1, root2) = solve_quadratic(opt.a, opt.b, opt.c);

    // TODO: Print results to a file
    println!("Root1: {:.4}", root1);
    println!("Root2: {:.4}", root2);
}
