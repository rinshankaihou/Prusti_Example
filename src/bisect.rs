// works with real Rust code
//     support: data structures, generics, traits, closure, unsafe code
//     probably not support: concurrency, I/O
// works on "unoptimized MIR", MIR that preserves information of the source code
// uses the compiler, run cargo-prusti
// proofs are macros that gets checked at compile time and does not change the generated code


use prusti_contracts::*;

/// A monotonically increasing discrete function, with domain [0, domain_size)
trait Function {
    #[pure]
    fn domain_size(&self) -> usize;

    #[pure]
    #[requires(x < self.domain_size())]
    fn eval(&self, x: usize) -> i32;
}

/// Find the `x` s.t. `f(x) == target`
#[requires(f.domain_size() < usize::MAX/2 as usize)]
#[ensures(if let Some(x) = result { f.eval(x) == target } else { true })]
fn bisect<T: Function>(f: &T, target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = f.domain_size();
    while low < high {
        body_invariant!(low < high && high <= f.domain_size());
        let mid = (low + high) / 2; //~ ERROR assertion might fail with "attempt to add with overflow"
        let mid_val = f.eval(mid);
        if mid_val < target {
            low = mid + 1;
        } else if mid_val > target {
            high = mid;
        } else {
            return Some(mid)
        }
    }
    None
}

pub fn main() {
}

