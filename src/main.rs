//! main.rs -> reads postfix notation mathematical expressions from a file and solves them
//!            Also, prints out the expressions written in infix notation along with solutions
//! Author: Brett Dale
//! Version: 1.0 (3/4/2021)

use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::process::exit;

///Expression struct to build our expressions from input file
#[derive(Debug)]
struct Expression {
    postfix: String,
    expr: Vec<f64>,
    infix: Vec<String>,
}

///Constructor for our Expression struct
impl Expression {
    fn new(input: String) -> Expression {
        Expression {
            postfix: input,
            expr: vec![],
            infix: vec![],
        }
    }

    ///Solves the postfix expression and creates the equivalent infix expression
    fn solve(&mut self) {
        let post = &self.postfix;
        let post = post.split_whitespace();
        for element in post{
            if element == "+" || element == "-" || element == "/" || element == "*" {
                let num_one = self.expr.pop().unwrap();
                let num_two = self.expr.pop().unwrap();
                let mut total: f64 = 0.0;

                if element == "+" {
                    total = num_one + num_two;
                }
                else if element == "-" {
                    total = num_two - num_one;
                }
                else if element == "*" {
                    total = num_one * num_two;
                }
                else if element == "/" {
                    total = num_two / num_one;
                }
                let second = self.infix.pop().expect("Invalid element in file");
                let first = self.infix.pop().expect("Invalid element in file");
                let temp = format!("({} {} {})", first, element, second);
                self.expr.push(total);
                self.infix.push(temp);
            }else {
                self.expr.push(element.parse::<f64>().expect("Failed to convert to f64"));
                self.infix.push(element.to_string());
            }
        }
    }
}

///Parses and handles command line argument, and contains the logic and code to run the program.
///If there is an error writing to the output file, main should print an appropriate error message.
fn main() {
    let args: Vec<String> = args().collect();
    let input_file = &args[2];
    println!("file to run: {}", args[1]);
    println!("input file is: {}", args[2]);
    println!("output file is: {}\n", args[3]);
    let mut expressions = build_expression_list(input_file).unwrap();
    println!("Each Expression after build_expression function: {:?}", expressions);
    solve_list(&mut expressions);
    println!("\nEach Expression after solve_list function: {:?}", expressions);
    expressions.reverse();
    println!("\nEach Expression after reverse function: {:?}", expressions);
    sort_list(&mut expressions);
    println!("\nEach Expression after sort_list function: {:?}", expressions);
}

///This function accepts a reference to a string slice representing the input file name
///and returns a ‘Result’ with a vector of expressions from the file or an appropriate error.
fn build_expression_list(file: &String) -> Result<Vec<Expression>, Error>{
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(&file);
    let mut container: Vec<Expression> = Vec::new();

    for line in reader.lines(){
        //println!("{}", line.unwrap());
        let mut postfix = String::new();

        for char in line.expect("Iterating through line String failed").chars(){
            if char == '+' || char == '-' || char == '/' || char == '*' {
                postfix.push(char);
            }
            else if char.is_numeric() {
                postfix.push(char);
            }
            else if char.is_whitespace() {
                if !postfix.is_empty() {
                    if !postfix.chars().last().unwrap().is_whitespace() {
                        postfix.push(' ');
                    }
                }
            }
        }
        if !postfix.is_empty() {
            //println!("{}", postfix);
            let temp: Expression = Expression::new(postfix);
            container.push(temp);
        }
    }
    Ok(container)
}

///Takes a reference to a vector of Expressions and solves them
fn solve_list(expressions: &mut Vec<Expression>) {
    for expression in expressions {
        expression.solve();
    }
}

///Takes a reference to a vector of Expressions and sorts them based on the value of the
///expressions solution
fn sort_list(expressions: &mut Vec<Expression>) {
    for i in 0..expressions.len(){
        for j in 0..expressions.len() - i - 1 {
            if expressions[j + 1].expr[0] < expressions[j].expr[0] {
                expressions.swap(j, j + 1);
            }
        }
    }
}
