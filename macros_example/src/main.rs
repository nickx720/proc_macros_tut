// https://dev.to/rogertorres/first-steps-with-rust-declarative-macros-1f8m
macro_rules! double {
    ($value: expr) => { $value * 2}
}

// $ and i are fixed and name follows rust variables convention

macro_rules! power {
    ($value:expr, cubed) => { $value.pow(3) };
    ($value:expr, squared) => { $value.pow(2) }
}

// expressions require one of these (=> , ;) -> can be used as delimiters

// Repetition * indicates any number of repetitons
// + indicates any number but atleast one
// ? indicates an optional with zero occurence

macro_rules! adder {
    // , before + (repetiton operator) -> separated for each repetiton
    ($left:expr, $($right:expr), +) => {{
        let mut total :i32 = 0;
        $(
            total +=$right;
            )+
            total
    }};
}

fn main() {
    dbg!(double!(7));
     dbg!(power!(3_i32,squared));
    dbg!(power!(3_i32,cubed));
    dbg!(adder!(1,2,3,4));
}
