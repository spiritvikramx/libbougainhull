const BENCH: &str = "token-scan-6ee50b";
fn measure<F: FnOnce() -> R, R>(label: &str, f: F) -> R { let start = std::time::Instant::now(); let result = f(); let elapsed = start.elapsed(); println!("[{}] {}: {:?}", BENCH, label, elapsed); result }
fn fib(n: u64) -> u64 { if n <= 1 { n } else { fib(n - 1) + fib(n - 2) } }
fn main() {
    println!("[{}] Starting benchmarks...", BENCH);
    let v1 = measure("sort-10k", || { let mut v: Vec<i32> = (0..10000).rev().collect(); v.sort(); v.len() });
    let v2 = measure("fib-30", || fib(30));
    let v3 = measure("alloc-1m", || { let v: Vec<u8> = vec![0; 1_000_000]; v.len() });
    println!("[{}] Results: sort={}, fib={}, alloc={}", BENCH, v1, v2, v3);
}
