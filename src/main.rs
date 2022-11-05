use std::io;
use std::fmt;
use std::collections::{LinkedList, HashMap};
use std::ops;
use substring::Substring;
//use std::fmt::Write;
//use std::rc::Rc;
//use std::cell::RefCell;

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

#[derive(Clone, Copy, PartialEq)]
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
            write!(f, "{:<19} ({}/{})", (self.num as f64)/(self.denom as f64), self.num, self.denom)

            /*if self.num / self.denom == 0 {
                write!(f, "{:>40}/{:<40} ({:^40})", (self.num % self.denom).abs(), self.denom, (self.num as f64)/(self.denom as f64))
            } else {
                write!(f, "{:>40}/{:<40} ({:^40}) [{:>81}/{:<40}]", self.num, self.denom, (self.num as f64)/(self.denom as f64), format!("{} {}", self.num / self.denom, (self.num % self.denom).abs()), self.denom)
            }*/
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Stack {
    pub data: LinkedList<Val>,
}

impl Stack {
    pub fn push(&mut self, v: Val) {
        self.data.push_front(v);
    }
    pub fn pop(&mut self) -> Option<Val> {
        self.data.pop_front()
    }
    pub fn top(&self) -> Option<Val> {
        match self.data.front() {
            Some(s) => {Some(s.clone())},
            None => {None}
        }
    }
    pub fn len(&self) -> Val {
        Val::Frac(Frac::new_int((self.data.len()) as i128))
    }
    pub fn new() -> Self {
        Self {
            data : LinkedList::<Val>::new()
        }
    }
}

impl ops::Add<Stack> for Stack {
    type Output = Stack;
    fn add(self, _rhs: Stack) -> Stack {
        let mut r = Stack::new();
        for e in self.data {
            r.push(e);
        }
        for e in _rhs.data {
            r.push(e);
        }
        return r;
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = "[".to_string();
        for e in &self.data {
            s = format!("{}{}, ", s, e);
        }
        s = format!("{}]", s);
        write!(f, "{}", s) 
    }
}

#[derive(Clone, PartialEq)]
pub struct Func {
    pub code: String,
}

impl Func {
    pub fn new(code: String) -> Self {
        Func {
            code,
        }
    } 
    /*pub fn append_tokens<'a>(&'a self, tokens: &mut LinkedList<&'a str>) -> Result<LinkedList<&str>, &str> {
        //let mut next = String::new();
        let mut new_tokens: LinkedList<String> = LinkedList::<String>::new(); // We're going to put tokens here to ensure they don't end up in the wrong order.
        new_tokens.push_front("".to_string());
        let mut parentheses_counter: isize = 0;
        for char in self.code.chars() {
            match char {
                ' ' | '\t' | '\r' => {
                    if parentheses_counter == 0 {
                        if new_tokens.front().ok_or("Turning the text into code went whack")?.len() != 0 {
                            //let x = next.clone();
                            //let y: String;
                            //next.clone();
                            //let mut o = "".to_string();
                            //write!(&mut o, "{}", next);
                            new_tokens.push_front("".to_string());
                        }
                        //next = String::new();
                    } else {
                        write!(&mut new_tokens.front().ok_or("Turning the text into code went whack")?, "{}", char);
                        //write!(&mut next, "{}", char);
                        //let x = format!("{}{}", next, char);
                        //next=x.as_str();
                    }
                }
                '(' => { parentheses_counter+=1; write!(&mut new_tokens.front().ok_or("Turning the text into code went whack")?, "{}", char); /*write!(&mut next, "{}", char);*/ }
                ')' => { parentheses_counter-=1; write!(&mut new_tokens.front().ok_or("Turning the text into code went whack")?, "{}", char); /*write!(&mut next, "{}", char);*/ }
                _ => {
                    write!(&mut new_tokens.front().ok_or("Turning the text into code went whack")?, "{}", char);
                    //write!(&mut next, "{}", char);
                }
            }
        }
        let mut ret = LinkedList::<&str>::new();
        for t in tokens {
            ret.push_front(t);
        }
        for t in new_tokens {
            ret.push_front(&t);
        }
        Ok(ret)
    }*/
    pub fn tokens(&self) -> Vec<String> {
        let mut next = "".to_string();
        let mut tokens: Vec<String> = Vec::<String>::new();
        let mut parentheses_counter: isize = 0;
        for char in self.code.chars() {
            match char {
                ' ' | '\t' | '\r' => {
                    if parentheses_counter == 0 {
                        if next != "" {
                            tokens.push(next);
                            next = "".to_string();
                        }
                    } else {
                        next = next.to_owned() + char.to_string().as_str();
                    }
                }
                '(' => {
                    parentheses_counter+=1;
                    next = next.to_owned() + char.to_string().as_str();
                }
                ')' => {
                    parentheses_counter-=1;
                    next = next.to_owned() + char.to_string().as_str();
                }
                _ => {
                    next = next.to_owned() + char.to_string().as_str();
                }
            }
        }
        if parentheses_counter == 0 {
            if next != "" {
                tokens.push(next);
            }
        }
        tokens
    }
}

impl ops::Add<Func> for Func {
    type Output = Func;
    fn add(self, _rhs: Func) -> Func {
        Func {
            code: format!("{} {}", self.code, _rhs.code).to_string(),
        }
    }
}

impl ops::Add<&Func> for Func {
    type Output = Func;
    fn add(self, _rhs: &Func) -> Func {
        Func {
            code: format!("{} {}", self.code, _rhs.code).to_string(),
        }
    }
}

impl ops::Mul<Frac> for Func {
    type Output = Func;
    fn mul(self, _rhs: Frac) -> Func {
        let mut r = Func::new("".to_string());
        for _i in 0.._rhs.int() {
            r = r + &self;
        }
        return r;
    }
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.code)
    }
}

#[derive(Clone, PartialEq)]
pub enum Val {
    Frac(Frac),
    Stack(Stack),
    Func(Func),
}

/*impl Copy for Val {}

impl Clone for Val {
    fn clone(&self) -> Val {
        match self {
            Val::Frac(f) => {Val::Frac(f.clone())},
            Val::Stack(s) => {
                let r: Stack = Stack::new();
                for v in &s.data {
                    r.push(v.clone());
                }
                return Val::Stack(r);
            },
            Val::Func(f) => {Val::Func(Func::new(f.code))}
        }
    }
}*/

impl fmt::Display for Val {
    fn fmt(&self, fo: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Val::Frac(fr) => {write!(fo, "{}", fr)}
            Val::Stack(s) => {write!(fo, "{}", s)}
            Val::Func(fu) => {write!(fo, "{}", fu)}
        }
    }
}

impl ops::Add<Val> for Val {
    type Output = Option<Val>;
    fn add(self, _rhs: Val) -> Option<Val> {
        match self {
            Val::Frac(frac0) => {
                match _rhs {
                    Val::Frac(frac1) => {Some(Val::Frac(frac0 + frac1))},
                    Val::Stack(_stack1) => {None},
                    Val::Func(_func1) => {None}
                }
            },
            Val::Stack(stack0) => {
                match _rhs {
                    Val::Frac(_frac1) => {None},
                    Val::Stack(stack1) => {Some(Val::Stack(stack0 + stack1))},
                    Val::Func(_func1) => {None}
                }
            },
            Val::Func(func0) => {
                match _rhs {
                    Val::Frac(_frac1) => {None},
                    Val::Stack(_stack1) => {None},
                    Val::Func(func1) => {Some(Val::Func(func0 + func1))}
                }
            }
        }
    }
}

impl ops::Sub<Val> for Val {
    type Output = Option<Val>;
    fn sub(self, _rhs: Val) -> Option<Val> {
        match self {
            Val::Frac(frac0) => {
                match _rhs {
                    Val::Frac(frac1) => {Some(Val::Frac(frac0 - frac1))},
                    Val::Stack(_stack1) => {None},
                    Val::Func(_func1) => {None}
                }
            },
            Val::Stack(_stack0) => {
                match _rhs {
                    Val::Frac(_frac1) => {None},
                    Val::Stack(_stack1) => {None},
                    Val::Func(_func1) => {None}
                }
            },
            Val::Func(_func0) => {
                match _rhs {
                    Val::Frac(_frac1) => {None},
                    Val::Stack(_stack1) => {None},
                    Val::Func(_func1) => {None}
                }
            }
        }
    }
}

impl ops::Mul<Val> for Val {
    type Output = Option<Val>;
    fn mul(self, _rhs: Val) -> Option<Val> {
        match self {
            Val::Frac(frac0) => {
                match _rhs {
                    Val::Frac(frac1) => {Some(Val::Frac(frac0 * frac1))},
                    Val::Stack(_stack1) => {None},
                    Val::Func(func1) => {Some(Val::Func(func1 * frac0))}
                }
            },
            Val::Stack(_stack0) => {
                match _rhs {
                    Val::Frac(_frac1) => {None},
                    Val::Stack(_stack1) => {None},
                    Val::Func(_func1) => {None}
                }
            },
            Val::Func(func0) => {
                match _rhs {
                    Val::Frac(frac1) => {Some(Val::Func(func0 * frac1))},
                    Val::Stack(_stack1) => {None},
                    Val::Func(_func1) => {None}
                }
            }
        }
    }
}

impl ops::Div<Val> for Val {
    type Output = Option<Val>;
    fn div(self, _rhs: Val) -> Option<Val> {
        match self {
            Val::Frac(frac0) => {
                match _rhs {
                    Val::Frac(frac1) => {Some(Val::Frac(frac0 / frac1))},
                    Val::Stack(_stack1) => {None},
                    Val::Func(_func1) => {None}
                }
            },
            Val::Stack(_stack0) => {
                match _rhs {
                    Val::Frac(_frac1) => {None},
                    Val::Stack(_stack1) => {None},
                    Val::Func(_func1) => {None}
                }
            },
            Val::Func(_func0) => {
                match _rhs {
                    Val::Frac(_frac1) => {None},
                    Val::Stack(_stack1) => {None},
                    Val::Func(_func1) => {None}
                }
            }
        }
    }
}

fn eval(code: &String, data: &mut HashMap<String, Val>, stack: &mut Stack) -> Result<Stack, String> {
    //if code.to_string() == "clear" { return Ok("\x1b[1;1H\x1b[2J\x1b[33mScreen Cleared\x1b[0m".to_string()); }
    //if code.to_string() == "" || code.to_string() == "help" { return Ok("Type numbers to push them to the stack, and type opperators to perform them on elements on the stack.\n\tEX:\t2 2 +\n\t\t4".to_string()); }
    //let binding = code.to_string().trim().to_string();
    //let mut tokens: Vec<&str> = binding.split(&[' ','\t','\n'][..]).collect();
    //let mut tokens: LinkedList<&str> = LinkedList::<&str>::new();
    let main_func: Func = Func::new(code.to_string());
    let mut tokens = main_func.tokens();
    tokens.reverse();
    tokens = tokens.iter().filter(|&token| token.to_string().as_str() != "").cloned().collect();
    //let mut stack: LinkedList<Frac> = LinkedList::<Frac>::new();
    //for token in &tokens {
    while tokens.len() > 0 {
        let token = tokens.pop().ok_or("Non-token tried to run")?;
        if token.chars().all(char::is_numeric) {
            stack.push(Val::Frac(Frac::new_int(i128::from_str_radix(&token, 10).expect("Some number"))));
        } else {
            match token.as_str() {
                //"" => {},
                "x" => {stack.pop();}, // Delete
                "+" => {
                    let n1 = stack.pop().ok_or("Second number in addition missing");
                    let n0 = stack.pop().ok_or("First number in addition missing");
                    stack.push((n0? + n1?).ok_or("Addition cannot be performed on provided types")?);
                }, // Add
                "-" => {
                    let n1 = stack.pop().ok_or("Second number in subtraction missing");
                    let n0 = stack.pop().ok_or("Firts number in subtraction missing");
                    stack.push((n0? - n1?).ok_or("Subtraction cannot be performed on provided types")?);
                }, // Subtract
                "*" => {
                    let n1 = stack.pop().ok_or("Second number in multiplication missing");
                    let n0 = stack.pop().ok_or("First number in multiplication missing");
                    stack.push((n0? * n1?).ok_or("Multiplication cannot be performed on provided types")?);
                }, // Multipluy
                "/" => {
                    let n1 = stack.pop().ok_or("Second number in division missing");
                    let n0 = stack.pop().ok_or("First number in division missing");
                    stack.push((n0? / n1?).ok_or("Division cannot be performed on provided types")?);
                }, // Divide
                ":" => {
                    let n = stack.pop().ok_or("Nothing to duplicate");
                    stack.push(n.clone()?);
                    stack.push(n?);
                }, // Duplicate
                "." => {
                    let n = stack.pop().ok_or("No number of elements to push back through provided");
                    let e = stack.pop().ok_or("Nothing to push backwards");
                    let mut rest: Stack = Stack::new();
                    match n? {
                        Val::Frac(f) => {
                            for _i in 0..f.int() {
                                rest.push(stack.pop().ok_or("Not enough space to push back that far")?);
                            }
                        },
                        Val::Stack(_s) => {
                            None.ok_or("Stacks are not numbers. How can I push back through [stack] elements?")?;
                        },
                        Val::Func(_s) => {
                            None.ok_or("Functions are not numbers. How can I push back through [function] elements?")?;
                        }
                    }
                    stack.push(e?);
                    for r in rest.data {
                        stack.push(r);
                    }
                    //stack.append(&mut rest);
                }, // Push backwards
                "[]" => {
                    stack.push(Val::Stack(Stack::new()));
                },
                "clear" => {
                    print!("\x1b[1;1H\x1b[2J\x1b[0m");
                },
                "print" => {
                    let v = stack.pop();
                    match v {
                        Some(some) => { print!("{}", some); },
                        None => {}
                    }
                },
                "println" => {
                    let v = stack.pop();
                    match v {
                        Some(Val::Func(func)) => {
                            if func.code.len() == 0 {
                                println!(""); // Empty function will print as a newline, I suppose.
                            } else {
                                println!("{}", func);
                            }
                        },
                        Some(some) => { println!("{}", some); },
                        None => { println!(""); }
                    }
                },
                "pchar" => {
                    let v = stack.pop();
                    match v {
                        Some(Val::Frac(f)) => { print!("{}", char::from_u32(f.int() as u32).ok_or("Cannot make char")?); },
                        Some(_) => {},
                        None => {},
                    }
                },
                "do" => {
                    let f = stack.pop();
                    match f {
                        Some(Val::Func(func)) => {
                            let mut new_tokens = func.tokens();
                            new_tokens.reverse();
                            for t in new_tokens {
                                tokens.push(t);
                            }
                        },
                        Some(_) => {},
                        None => {},
                    }
                },
                "do_on" | "`" => {
                    let f = stack.pop();
                    let s = stack.pop();
                    match f {
                        Some(Val::Func(func)) => {
                            match s {
                                Some(Val::Stack(mut other_stack)) => {
                                    stack.push(Val::Stack(eval(&func.code, data, &mut other_stack)?));
                                },
                                Some(_) => {},
                                None => {}
                            }
                        },
                        Some(_) => {},
                        None => {},
                    }
                },
                "pop" => {
                    let s = stack.pop();
                    match s {
                        Some(Val::Stack(other_stack)) => {
                            let v = other_stack.clone().pop();
                            stack.push(v.ok_or("Cannot pop empty stack!")?);
                        },
                        Some(_) => {None.ok_or("Cannot pop from non-stack!")?;},
                        None => {None.ok_or("No stack to pop from!")?;}
                    }
                },
                "size" => { // Stack only
                    let v = stack.pop();
                    match v {
                        Some(Val::Stack(s)) => {
                            let x = s.data.len().try_into();
                            match x {
                                Ok(ok) => {stack.push(Val::Frac(Frac::new_int(ok)));},
                                Err(_) => {None.ok_or("Number conversion error in size test")?;}
                            }
                        },
                        Some(_) => {None.ok_or("You can only check the size of stacks.")?;},
                        None => {None.ok_or("Nothing to measure")?;}
                    }
                },
                "=" | "==" => {
                    let a = stack.pop().ok_or("You need to have two things to make a comparison")?;
                    let b = stack.pop().ok_or("You need to have two things to make a comparison")?;
                    if a == b {
                        stack.push(Val::Frac(Frac::new_int(1)));
                    } else {
                        stack.push(Val::Frac(Frac::new_int(0)));
                    }
                },
                ">" => {},
                "<" => {},
                ">=" => {},
                "<=" => {},
                "not" => {},
                "and" => {},
                "or" => {}
                "xor" => {},
                "if" => {},
                "while" => {},
                "rand" | "?" => {},
                _ => {
                    if token.substring(0, 1) == "<" { // load var
                        let var_name = token.substring(1, token.len());
                        //println!("Load var {}", var_name);
                        let v = data.get(var_name).ok_or("Load a variable from the data");
                        match v {
                            Ok(d) => {stack.push(d.clone());},
                            Err(_e) => {stack.push(Val::Frac(Frac::new_int(0)));}
                        }
                    } else if token.substring(0, 1) == ">" { // set var 
                        let var_name = token.substring(1, token.len());
                        //println!("Save var {}", var_name);
                        let v = stack.pop().ok_or("Number to save to variable");
                        match v {
                            Ok(d) => {data.insert(var_name.to_string(), d);},
                            Err(_e) => {}
                        }
                    } else if token.substring(0, 1) == "(" && token.substring(token.len()-1, token.len()) == ")" {
                        stack.push(Val::Func(Func::new(token.substring(1, token.len()-1).to_string())));
                    } else {
                        println!("{} is not a valid token", token);
                    }
                }
            }
        }
    }
    /*match stack.pop() {
        Some(v) => {
            data.insert("".to_string(), v.clone());
            return Ok(v);
        }
        None => {return Err("Nothing to print left on the stack".to_string());}
    }*/
    return Ok(stack.clone());
    //Ok(stack.pop().ok_or("Nothing to print left on stack").to_string())
}

fn main() {
    println!("Stackulator: The Stack-Based Calculator");
    let mut code: String;
    let mut data: HashMap<String, Val> = HashMap::<String, Val>::new();
    data.insert("".to_string(), Val::Frac(Frac::new_int(0)));
    let mut last_line: String = "help".to_string();
    while last_line != "quit" && last_line != "exit" && last_line != "stop" {
        code = String::new();    // Clear out any old commands
        io::stdin().read_line(&mut code).expect("A simple prompt to process");    // Take in input
        code = code.trim().to_string();    // Remove the newlines
        if code == "quit" || code == "exit" || code == "stop" {
            println!("Bye :)");
            last_line = code;
        } else if code == "" {
            let mut stack = Stack::new();
            let r = eval(&last_line, &mut data, &mut stack);
            match r {
                Ok(o) => {
                    match o.top() {
                        Some(v) => {
                            data.insert("".to_string(), v.clone());
                            println!("{}", v);
                        }
                        None => {
                            println!("Nothing to print left on the stack");
                        }
                    }
                },
                Err(e) => {
                    println!("ERROR: {}", e);
                }
            }
        } else {
            let mut s: Stack = Stack::new();
            let r = eval(&code, &mut data, &mut s);
            match r {
                Ok(o) => {
                    match o.top() {
                        Some(v) => {
                            data.insert("".to_string(), v.clone());
                            println!("{}", v);
                        }
                        None => {
                            println!("Nothing to print left on the stack");
                        }
                    }
                },
                Err(e) => {
                    println!("ERROR: {}", e);
                }
            }
            last_line = code;
        }
    }
}
