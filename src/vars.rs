pub fn run() {
    let name = "Tomas";
    let mut age = 32;
    println!("My name is {} and i am {}", name, age);

    age = 33;
    println!("My name is {} and i am {}", name, age);

    //Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple vars
    let (my_name, my_age) = ("Brad", 32);
    println!("{} is {}", my_name, my_age);
}
