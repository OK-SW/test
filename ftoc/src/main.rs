use std::io;

fn main() {

    loop {
        println!("choose the mode");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");

        let mut cmd = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");

        let cmd: u32 = match cmd.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if cmd == 1{
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let mut output: f64 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) =>continue,
            };



            output = (output - 32.0f64) * (5.0f64 / 9.0f64);

            println!("{output}")
        } else if cmd == 2 {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let mut output: f64 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) =>continue,
            };

            output = output * (9.0f64 / 5.0f64) + 32.0f64;

            println!("{output}")
        } else if cmd == 3{
            println!("quit");
            break;
        } else {
            println!("wrong cmd");
            continue;
        }
    }
}
