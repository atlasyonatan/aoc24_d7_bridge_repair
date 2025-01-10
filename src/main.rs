pub mod operators;

use operators::{Add, Concat, Inverse, Mul};
use std::io::{self};

fn main() {
    let lines = io::stdin().lines().map(Result::unwrap);

    let equations = lines.map(|line| {
        let (before, after) = line.split_once(':').unwrap();
        let test_value: u64 = before.parse().unwrap();
        let remaining: Vec<u16> = after
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        (test_value, remaining)
    });

    let mut operators: Vec<&dyn Inverse<u64, u64, u64>> = vec![&Add, &Mul];
    let (possible_equations, not_possible_equations): (Vec<_>, Vec<_>) = equations
        .partition(|eq| is_equation_possible_with_operators(eq.0, &eq.1[..], operators.as_slice()));

    let result = possible_equations.into_iter().map(|eq| eq.0).sum::<u64>();
    println!("part 1: {}", result);

    operators.push(&Concat);
    let new_possible_equation = not_possible_equations
        .into_iter()
        .filter(|eq| is_equation_possible_with_operators(eq.0, &eq.1[..], operators.as_slice()));

    let result = result + new_possible_equation.map(|eq| eq.0).sum::<u64>();
    println!("part 2: {}", result);
}

fn is_equation_possible_with_operators(
    goal: u64,
    numbers: &[u16],
    operators: &[&dyn Inverse<u64, u64, u64>],
) -> bool {
    if numbers.len() == 0 {
        return goal == 0;
    }

    let last_number = *numbers.last().unwrap() as u64;
    let rest_of_numbers = &numbers[0..numbers.len() - 1];

    operators
        .into_iter()
        .any(|operator| match operator.inverse(goal, last_number) {
            Some(new_goal) => {
                is_equation_possible_with_operators(new_goal, rest_of_numbers, operators)
            }
            None => false,
        })
}
