#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

#[nexus_rt::main]
fn main() {
    let n = 10;
    let result = fib(n);

    assert_eq!(result, 55);
}

fn fib(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut sum;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }
    b
}
