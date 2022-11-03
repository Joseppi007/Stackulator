use std::io;
use std::fmt;
use std::collections::{LinkedList, HashMap};
use std::ops;
use substring::Substring;

fn min(x: i128, y: i128) -> i128 {
    if x < y {
        return x;
    }
    return y;
}

/// Finds the greatest common denominator
/// ```
/// assert_eq!(gcd(69, 420), 3);
/// assert_eq!(gcd(621, 621), 621);
/// assert_eq!(gcd(666, 665), 1);
/// ```
fn gcd(x: i128, y: i128) -> i128 {
    //if x == 0 || y == 0 {return 0;}
    //if x > y {
    //    let g = gcd(x%y, y);
    //    if g == 0 {
    //        return y;
    //    }
    //    g
    //} else {
    //    let g = gcd(y%x, x);
    //    if g == 0 {
    //        return x;
    //    }
    //    g
    //}

    let mut g = min(x, y);
    let mut last_g = min(x, y);
    let mut a = x;
    let mut b = y;
    while g != 0 {
        if a == 0 || b == 0 {return last_g;}
        if a < b || b < 0 {
            let c = b;
            b = a;
            a = c;
        }
        a = a%b;
        last_g = g;
        g = a;
    }
    return last_g;
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
    /*
    pub fn add(&self, other: Frac) -> Self {
        Self::new(
            self.num * other.denom + other.num * self.denom,
            self.denom * other.denom,
        )
    }
    pub fn subtract(&self, other: Frac) -> Self {
        Self::new(
            self.num * other.denom - other.num * self.denom,
            self.denom * other.denom,
        )
    }
    pub fn multiply(&self, other: Frac) -> Self {
        Self::new(
            self.num * other.num,
            self.denom * other.denom,
        )
    }
    pub fn divide(&self, other: Frac) -> Self {
        Self::new(
            self.num * other.denom,
            self.denom * other.num,
        )
    }
    */
    pub fn int(&self) -> i128 {
        return self.num / self.denom;
    }
    pub fn simplify(&self) -> Self {
        if self.num == 0 {return Frac::new_unchecked(0, 1);}
        if self.denom == 0 {
            if self.num > 0 {return Frac::new_unchecked(1, 0);}
            if self.num < 0 {return Frac::new_unchecked(-1, 0);}
            return Frac::new_unchecked(0, 0);
        }
        let g: i128;
        //if self.denom < 0 && self.num < 0 {g = -gcd(-self.num, -self.denom);}
        /*else {*/g = gcd(self.num, self.denom);//}
        let r = Frac::new_unchecked(self.num / g, self.denom / g);
        if r.denom < 0 && r.num > 0 {return Frac::new_unchecked(-r.num, -r.denom);}
        return r;
    }
}

impl ops::Add<Frac> for Frac {
    type Output = Frac;
    fn add(self, _rhs: Frac) -> Frac {
        Self::new(
            self.num * _rhs.denom + _rhs.num * self.denom,
            self.denom * _rhs.denom,
        )
    }
}

impl ops::Sub<Frac> for Frac {
    type Output = Frac;
    fn sub(self, _rhs: Frac) -> Frac {
        Self::new(
            self.num * _rhs.denom - _rhs.num * self.denom,
            self.denom * _rhs.denom,
        )
    }
}

impl ops::Mul<Frac> for Frac {
    type Output = Frac;
    fn mul(self, _rhs: Frac) -> Frac {
        Self::new(
            self.num * _rhs.num,
            self.denom * _rhs.denom,
        )
    }
}

impl ops::Div<Frac> for Frac {
    type Output = Frac;
    fn div(self, _rhs: Frac) -> Frac {
        Self::new(
            self.num * _rhs.denom,
            self.denom * _rhs.num,
        )
    }
}

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.denom == 1 {
            write!(f, "{}", self.num)
        } else if self.denom == 0 {
            if self.num > 0 {
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

fn eval(code: &String, data: &mut HashMap<String, Frac>) -> Result<String, String> {
    if code.to_string() == "clear" { return Ok("\x1b[1;1H\x1b[2J\x1b[33mScreen Cleared\x1b[0m".to_string()); }
    if code.to_string() == "" || code.to_string() == "help" { return Ok("Type numbers to push them to the stack, and type opperators to perform them on elements on the stack.\n\tEX:\t2 2 +\n\t\t4".to_string()); }
    let binding = code.to_string().trim().to_string();
    let mut tokens: Vec<&str> = binding.split(&[' ','\t','\n'][..]).collect();
    tokens = tokens.iter().filter(|&&token| token != "").cloned().collect();
    let mut stack: LinkedList<Frac> = LinkedList::<Frac>::new();
    for token in &tokens {
        if token.chars().all(char::is_numeric) {
            stack.push_front(Frac::new_int(i128::from_str_radix(token, 10).expect("Some number")));
        } else {
            match *token {
                //"" => {},
                "x" => {stack.pop_front();}, // Delete
                "+" => {
                    let n1 = stack.pop_front().ok_or("Second number in addition missing");
                    let n0 = stack.pop_front().ok_or("First number in addition missing");
                    stack.push_front(n0? + n1?);
                }, // Add
                "-" => {
                    let n1 = stack.pop_front().ok_or("Second number in subtraction missing");
                    let n0 = stack.pop_front().ok_or("Firts number in subtraction missing");
                    stack.push_front(n0? - n1?);
                }, // Subtract
                "*" => {
                    let n1 = stack.pop_front().ok_or("Second number in multiplication missing");
                    let n0 = stack.pop_front().ok_or("First number in multiplication missing");
                    stack.push_front(n0? * n1?);
                }, // Multipluy
                "/" => {
                    let n1 = stack.pop_front().ok_or("Second number in division missing");
                    let n0 = stack.pop_front().ok_or("First number in division missing");
                    stack.push_front(n0? / n1?);
                }, // Divide
                ":" => {
                    let n = stack.pop_front().ok_or("Nothing to duplicate");
                    stack.push_front(n?);
                    stack.push_front(n?);
                }, // Duplicate
                "." => {
                    let n = stack.pop_front().ok_or("No number of elements to push back through provided");
                    let e = stack.pop_front().ok_or("Nothing to push backwards");
                    let mut rest: LinkedList<Frac> = LinkedList::<Frac>::new();
                    for _i in 0..n?.int() {
                        rest.push_front(stack.pop_front().ok_or("Not enough space to push back that far")?);
                    }
                    stack.push_front(e?);
                    for r in rest {
                        stack.push_front(r);
                    }
                    //stack.append(&mut rest);
                }, // Push backwards
                _ => {
                    if token.substring(0, 1) == "<" { // load var
                        let var_name = token.substring(1, token.len());
                        //println!("Load var {}", var_name);
                        let v = data.get(var_name).ok_or("Load a variable from the data");
                        match v {
                            Ok(d) => {stack.push_front(*d);},
                            Err(_e) => {stack.push_front(Frac::new_int(0));}
                        }
                    } else if token.substring(0, 1) == ">" { // set var 
                        let var_name = token.substring(1, token.len());
                        //println!("Save var {}", var_name);
                        let v = stack.pop_front().ok_or("Number to save to variable");
                        match v {
                            Ok(d) => {data.insert(var_name.to_string(), d);},
                            Err(_e) => {}
                        }
                    } else {
                        println!("{} is not a valid token", token);
                    }
                }
            }
        }
    }
    match stack.pop_front() {
        Some(v) => {
            data.insert("".to_string(), v);
            return Ok(v.to_string());
        }
        None => {return Err("Nothing to print left on the stack".to_string());}
    }
    //Ok(stack.pop_front().ok_or("Nothing to print left on stack").to_string())
}

fn main() {
    println!("Stackulator: The Stack-Based Calculator");
    let mut code: String;
    let mut data: HashMap<String, Frac> = HashMap::<String, Frac>::new();
    data.insert("".to_string(), Frac::new_int(0));
    let mut last_line: String = "help".to_string();
    while last_line != "quit" && last_line != "exit" && last_line != "stop" {
        code = String::new();    // Clear out any old commands
        io::stdin().read_line(&mut code).expect("A simple prompt to process");    // Take in input
        code = code.trim().to_string();    // Remove the newlines
        if code == "quit" || code == "exit" || code == "stop" {
            println!("Bye :)");
        } else if code == "" {
            let r = eval(&last_line, &mut data);
            match r {
                Ok(ok) => {println!("{}", ok);}
                Err(e) => {println!("ERROR: {}", e);}
            }
        } else {
            let r = eval(&code, &mut data);
            match r {
                Ok(ok) => {println!("{}", ok);}
                Err(e) => {println!("ERROR: {}", e);}
            }
            last_line = code;
        }
    }
}
