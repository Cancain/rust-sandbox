pub fn run() {
    //Print to console
    println!("Hello from print.rs file");

    //Basic formatting
    println!("{} is from {}", "Dowie", "Gävle");

    //Positional arguments
    println!(
        "{0} is form {1} and {0} likes to {2}",
        "Dowie", "Gävle", "code"
    );

    //Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "Dowie",
        activity = "Overwatch"
    );

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic math
    println!("10 + 10 = {}", 10 + 10);
}
