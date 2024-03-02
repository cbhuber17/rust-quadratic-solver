// TODO: Argparse a, b, c

// mod utils;
use rust_quadratic_solver::input::get_input;
use rust_quadratic_solver::quadratic::solve_quadratic;

fn main() {
    let a: f32 = get_input("a".to_string());
    if a == 0.0 {
        println!("Error: 'a' cannot be zero.");
        panic!();
    }
    let b: f32 = get_input("b".to_string());
    let c: f32 = get_input("c".to_string());

    let (root1, root2) = solve_quadratic(a, b, c);

    println!("Root1: {:.4}", root1);
    println!("Root2: {:.4}", root2);
}

// ------------------------------------------------------------------------------
