#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for inp in inputs {
        match inp {
            CalculatorInput::Value(num) => stack.push(*num),
            _ => {
                if stack.len() < 2 {
                    return None;
                }
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                match inp {
                    CalculatorInput::Add => stack.push(lhs + rhs),
                    CalculatorInput::Subtract => stack.push(lhs - rhs),
                    CalculatorInput::Multiply => stack.push(lhs * rhs),
                    CalculatorInput::Divide => stack.push(lhs / rhs),
                    _ => return None,
                }
            }
        }
    }
    if stack.len() > 1 {
        return None;
    }
    return stack.pop();
}
