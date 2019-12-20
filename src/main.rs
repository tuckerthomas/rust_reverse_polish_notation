use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        println!("Reverse Polish Notation Calculator");
    
        print!("Enter Calculation: ");
        stdout().flush().expect("Could not flush stdout");
    
        let mut input = String::new();
    
        stdin().read_line(&mut input).expect("Could not get input");

        let input = input.trim();
    
        println!("Entered: '{}' Result: {}\n", input, rpn_calc(&input));
    }
}

fn rpn_calc(s: &str) -> f64 {
    let params_iter = s.split_whitespace();
    let mut nums: Vec<f64> = Vec::new();

    for param in params_iter {
        
        // Fix double parse
        if param.parse::<f64>().is_ok() {
            println!("{} is a number", param);
            nums.push(param.parse::<f64>().unwrap())
        } else if param.is_empty() {
            // I don't think it ever gets here...
            println!("End of Sequence");
            break;
        } else {
            match param {
                "+" => {
                    let num1 = nums.pop().unwrap();
                    let num2 = nums.pop().unwrap();
                    nums.push(num1 + num2);
                    continue;
                },
                "-" => {
                    let num1 = nums.pop().unwrap();
                    let num2 = nums.pop().unwrap();
                    nums.push(num1 - num2);
                    continue;
                }
                "*" | "x" => {
                    let num1 = nums.pop().unwrap();
                    let num2 = nums.pop().unwrap();
                    nums.push(num1 * num2);
                    continue;
                }
                "/" => {
                    let num1 = nums.pop().unwrap();
                    let num2 = nums.pop().unwrap();
                    nums.push(num1 / num2);
                    continue;
                }
                "=" => {
                    break;
                },
                _ => {
                    println!("Invalid Sequence");
                    return 0.0;
                }
            }
        }
    }

    return nums.pop().expect("Something's Wrong");
}