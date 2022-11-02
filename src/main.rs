use std::io;
use std::fmt;
use std::collections::LinkedList;

fn gcd(x: i128, y: i128) -> i128 {
    if x == 0 || y == 0 {return 0;}
    if x > y {
        let g = gcd(x%y, y);
        if g == 0 {
            return y;
        }
        g
    } else {
        let g = gcd(y%x, x);
        if g == 0 {
            return x;
        }
        g
    }
}

#[derive(Clone, Copy)]
pub struct Frac {
    pub num: i128,
    pub denom: i128
}

impl Frac {
    pub fn new(num: i128, denom: i128) -> Self {
        Self {
            num,
            denom,
        }.simplify()
    }
    fn new_unchecked(num: i128, denom: i128) -> Self {
        Self {
            num,
            denom,
        }
    }
    pub fn new_int(num: i128) -> Self {
        Self {
            num,
            denom:1,
        }
    }
    pub fn add(&self, other: Frac) -> Self {
        Self::new(
            self.num * other.denom + other.num * self.denom,
            self.denom * other.denom,
        )
    }
    pub fn simplify(&self) -> Self {
        let g = gcd(self.num, self.denom);
        return Frac::new_unchecked(self.num / g, self.denom / g);
    }
}

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.denom == 1 {
            write!(f, "{}", self.num)
        } else if self.denom == 0 {
            if self.num < 0 {
                write!(f, "Infinity")
            } else if self.num < 0 {
                write!(f, "-Infinity")
            } else {
                write!(f, "NaN")
            }
        } else {
            write!(f, "{}/{}", self.num, self.denom)
        }
    }
}

fn eval(code: &String) -> String {
    if code.to_string() == "clear" { return "\x1b[1;1H\x1b[2J\x1b[33mScreen Cleared\x1b[0m".to_string(); }
    if code.to_string() == "" || code.to_string() == "help" { return "Type numbers to push them to the stack, and type opperators to perform them on elements on the stack.\n\tEX:\t2 2 +\n\t\t4".to_string(); }
    let binding = code.to_string().trim().to_string();
    let tokens: Vec<&str> = binding.split(' ').collect();
    let mut stack: LinkedList<Frac> = LinkedList::<Frac>::new();
    for token in &tokens {
        if token.chars().all(char::is_numeric) {
            stack.push_front(Frac::new_int(i128::from_str_radix(token, 10).expect("A number")));
        } else {
            match *token {
                "x" => {stack.pop_front();}, // Delete
                "+" => {
                    let n1 = stack.pop_front().expect("The second number for addition");
                    let n0 = stack.pop_front().expect("The first number for addition");
                    stack.push_front(n0.add(n1));
                }, // Add
                "-" => {
                    let n1 = stack.pop_front().expect("The second number for addition");
                    let n0 = stack.pop_front().expect("The first number for addition");
                    stack.push_front(n0.add(n1));
                }, // Subtract
                "*" => {
                    let n1 = stack.pop_front().expect("The second number for addition");
                    let n0 = stack.pop_front().expect("The first number for addition");
                    stack.push_front(n0.add(n1));
                }, // Multipluy
                "/" => {
                    let n1 = stack.pop_front().expect("The second number for addition");
                    let n0 = stack.pop_front().expect("The first number for addition");
                    stack.push_front(n0.add(n1));
                }, // Divide
                ":" => {
                    let n1 = stack.pop_front().expect("The second number for addition");
                    let n0 = stack.pop_front().expect("The first number for addition");
                    stack.push_front(n0.add(n1));
                }, // Duplicate
                "." => {
                    let n1 = stack.pop_front().expect("The second number for addition");
                    let n0 = stack.pop_front().expect("The first number for addition");
                    stack.push_front(n0.add(n1));
                }, // Push backwards
                "" => {},
                _ => {println!("{} is not a valid token", token);}
            }
        }
    }
    stack.pop_front().expect("Print the top of the stack").to_string()
}

fn main() {
    println!("Stackulator: The Stack-Based Calculator");
    let mut code: String = String::new();
    while code != "quit" && code != "exit" && code != "stop" {
        code = String::new();    // Clear out any old commands
        io::stdin().read_line(&mut code).expect("A simple prompt to process");    // Take in input
        code = code.trim().to_string();    // Remove the newlines
        if code == "quit" || code == "exit" || code == "stop" {
            println!("Bye :)");
        } else {
            println!("{}", eval(&code));
        }
    }
}
