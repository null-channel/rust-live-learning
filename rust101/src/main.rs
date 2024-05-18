use clap::{Parser, Subcommand};

fn main() {
    println!("Hello, world!");
    let my_struct = MyStruct { name: String::from("me"), age: 32 };
    println!("my struct: {:?}", my_struct);

    let x = 5;
    needs_i64(x);

    let cli = Cli::parse();

    if let Some(name) = cli.name {
        println!("our name: {}", name);
    } else {
        println!("no name passed in");
    }

}

// basic function
fn needs_i64(age: i64) {
    println!("age: {:?}",age)
}

#[derive(Debug,PartialEq, Eq)]
struct MyStruct {
    name: String,
    age: i32,
}


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    #[arg(short, long)]
    name: Option<String>,
}
