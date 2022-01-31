// approximates the next point through a slope field using n subdivisions

#[macro_use]
extern crate clap;

use clap::Arg;
use rhai::{Engine, Scope};

fn main() {
    let engine = Engine::new();

    let matches = app_from_crate!()
        .arg(
            Arg::new("diffeq")
                .required(true)
                .value_name("diffeq")
                .long("de")
                .help("The Differential Equation f(x,y)"),
        )
        .arg(
            Arg::new("subdivs")
                .required(true)
                .value_name("subdivs")
                .short('n')
                .help("The number of subsections to use"),
        )
        .arg(
            Arg::new("startx")
                .required(true)
                .value_name("startx")
                .short('x')
                .help("The starting x point"),
        )
        .arg(
            Arg::new("starty")
                .required(true)
                .value_name("starty")
                .short('y')
                .help("The starting y point"),
        )
        .arg(
            Arg::new("endx")
                .required(true)
                .value_name("endx")
                .short('X')
                .help("The ending x point"),
        )
        .get_matches();

    let diff_eq = engine
        .compile(matches.value_of("diffeq").expect("clap failure"))
        .expect("failed to compile rhai AST from diff eq");
    let subdivs: usize = matches
        .value_of("subdivs")
        .expect("clap failure")
        .parse()
        .expect("could not parse subdivs to usize");
    let mut prev_x: f64 = matches
        .value_of("startx")
        .expect("clap failure")
        .parse()
        .expect("could not parse startx to f64");
    let mut prev_y: f64 = matches
        .value_of("starty")
        .expect("clap failure")
        .parse()
        .expect("could not parse starty to f64");
    let end_x: f64 = matches
        .value_of("endx")
        .expect("clap failure")
        .parse()
        .expect("could not parse endx to f64");

    let step = (end_x - prev_x) / subdivs as f64;

    let mut curr_x;
    let mut curr_y;

    while prev_x < end_x {
        let mut scope = Scope::new();

        scope.push_constant("x", prev_x);
        scope.push_constant("y", prev_y);

        let diff_eq_result = engine
            .eval_ast_with_scope::<f64>(&mut scope, &diff_eq)
            .expect("failed to calculate the result of the diffeq function");

        curr_x = prev_x + step;
        curr_y = prev_y + diff_eq_result * step;

        println!(
            "next point for ({}, {}) => ({}, {})",
            prev_x, prev_y, curr_x, curr_y
        );

        prev_x = curr_x;
        prev_y = curr_y;
    }
}
