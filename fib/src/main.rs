fn main() {
  fib(11)
}

fn add(num_1: &mut u64, num_2: &u64) {
  *num_1 = *num_1 + num_2;
}

fn fib(n: u64) {
    let mut i: u64 = 0;
    let mut sum: u64 = 0;

    loop {
        if i > n {
            break;
        }

        add(&mut sum, &i);
        println!("{} {}", i, sum);
        i = i + 1;
    }
}
