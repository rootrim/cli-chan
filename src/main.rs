use colored::Colorize;

#[tokio::main]
async fn main() {
    let name = "JoJo".blue().bold();
    let message = "Goodbye".red().italic();
    println!("{message}, {name}!");
}
