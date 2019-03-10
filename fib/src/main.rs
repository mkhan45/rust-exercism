fn main() {
    println!("{}", fib(93));
}

fn fib(n: usize) -> usize{
    let mut fib_arr = vec![1, 1];

    for i in fib_arr.len()..n{
        fib_arr.push(fib_arr[i-1] + fib_arr[i-2]);
    }

    return fib_arr[n-1];
}
