use num_integer::Integer;
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

    let result: u64 = equations
        .filter(|eq| is_equation_possible(eq.0, &eq.1[..]))
        .map(|eq| eq.0)
        .sum();

    println!("part 1: {}", result);
}

fn is_equation_possible(goal: u64, numbers: &[u16]) -> bool {
    if numbers.len() == 0 {
        return goal == 0;
    }

    let last_number = *numbers.last().unwrap() as u64;
    let rest_of_numbers = &numbers[0..numbers.len() - 1];
    //insert + end
    let possible_with_addition = match goal.checked_sub(last_number) {
        Some(new_goal) => is_equation_possible(new_goal, rest_of_numbers),
        None => false,
    };

    if possible_with_addition {
        return true;
    }

    //insert * end
    let possible_with_multiplication = match goal.div_rem(&last_number) {
        (new_goal, 0) => is_equation_possible(new_goal, rest_of_numbers),
        _ => false,
    };

    if possible_with_multiplication {
        return true;
    }

    return false;
}
