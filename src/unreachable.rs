

pub fn main() {
    let a=Some(1);
    match a {
    Some(x) => { /* do stuff with x */ },
    None  => { unreachable!() },
    }
}