use std::io;

fn main() {
    loop {
        println!("input the num");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        fibo(0, 1, input);

    }
}

fn fibo(f1: u32, f2: u32, counting: u32) {
    if counting == 0 {
        println!("{f1}");
    } else {
        fibo(f2, f1 + f2, counting - 1);
    }
}
