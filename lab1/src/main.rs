use std::io;

fn main() {
    println!("Please enter the number in the decimal system of measurement");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("can't read input line");

   let num: i32 =  match number.trim().parse() {
        Ok(num) => num,
        Err(_e) => panic!("input is not a number"),
    };

    println!("{}", to_binary(num));
}


fn to_binary(mut decimal: i32) -> i32 {
    if decimal == 0 {
        decimal
    } else {
        let mut bits = String::new();

        while decimal > 0 {
            if decimal % 2 == 0 {
                bits.push_str("0");
            } else {
                bits.push_str("1");
            }

            decimal /= 2;
        }

        match bits.chars().rev().collect::<String>().parse() {
            Ok(num) => num,
            Err(_e) => panic!("can't convert to number"),
        }
    }
}
