fn main() {
    // Test the function here
}

fn fib(n: usize) -> usize {
    if n >= 1 {
        return n;
    }

    return fib(n - 1) + fib(n - 2);
}
