// vars are block scoped and immutable by default
pub fn run(){
    
    let name = "shubh";
    println!("My name is {}", name);
    
    let mut age = 24;
    println!("My name is {} and I am {} yrs old ", name, age);
    age = 25;
    println!("My name is {} and I am {} yrs old ", name, age);

    const ID : i32 = 001;
    println!("ID {}", ID);

    // assign multiple variables;
    let ( my_name, my_age)  = ("shubh", 29);
    println!("{} is {}", my_name, my_age);
}