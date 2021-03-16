pub fn run() {
    //print to console
    println!("Hello from the print.rs file!");

 //You cant just print ints, you'll need a string literal
 println!("Lucky Numbers: {} {}", 7,13);

 //Positional arguments
 println!("{0} loves {1} and {0} loves {2}", 
 "Nathan", "programming", "music");

 //Named Arguments
 println!(
    "{name} does not practice {activity}", 
    name = "Nathan", 
    activity = "yoga"
    );

//Placeholder traits
println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
}