use std::io::stdin;
use task6::run;

fn main() {
    println!("Please enter arithmetic expression in infix form:");
    let mut data = String::new();
    stdin().read_line(&mut data).unwrap();

    match run(&data) {
        Ok((postfix, result)) => {
            println!("postfix notation: {}", postfix);
            println!("evaluation result: {}", result);
        }
        Err(e) => eprintln!("{}", e)
    }
}
