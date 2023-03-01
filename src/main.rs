fn main() {
    loop {
        println!("Calculator!");
        let mut line = String::new();
        let _b1 = std::io::stdin().read_line(&mut line).unwrap();
        let input: Vec<&str> = line.split_whitespace().collect();
        println!("{}", input.len());
        if input[0] == "quit" {
            break;
        } else if input.len() == 3 {
            calc(input);        
        } else {
            println!("Not correct format!")
        }
    }
    println!("Bye bye!")
}

fn calc(input: Vec<&str>) -> () {
    let operator = input[1];
    let first = input[0].parse::<f32>().unwrap();
    let second = input[2].parse::<f32>().unwrap();
    match operator{
        "+"=>println!("{}",first+second),
        "-"=>println!("{}",first-second),
        "*"=>println!("{}",first*second),
        "/"=>println!("{}",first/second),
        _=>println!("No valid operator!")
    }
}
