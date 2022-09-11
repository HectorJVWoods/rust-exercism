use std::error::Error;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

/// Apply a function to the two top values on a stack.
/// If there are not enough values on the stack, return None
/// Otherwise, return the result of the function applied to the top two values
pub fn pop_twice_and_do<T>(stack: &mut Vec<T>, f: fn(T, T) -> T) -> Option<T> {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(a), Some(b)) => Some(f(a, b)),
        _ => None,
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack : Vec<i32> = Vec::new();
    for input in inputs {
        let result = match input {
            CalculatorInput::Add =>  pop_twice_and_do(&mut stack, |a, b| a + b),
            CalculatorInput::Subtract => pop_twice_and_do(&mut stack, |a, b| a - b),
            CalculatorInput::Multiply => pop_twice_and_do(&mut stack, |a, b| a * b),
            CalculatorInput::Divide => pop_twice_and_do(&mut stack, |a, b| a / b),
            CalculatorInput::Value(x) => Some(*x),
        };
        match result {
            Some(x) => stack.push(x),
            None => return None,
        }
    }
    match stack.len() {
        0 => None,
        1 => Some(stack.pop().unwrap()),
        _ => None,
    }
}