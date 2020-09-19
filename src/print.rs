pub fn run() {
    // print to console 
    println!("hello from print rs file");
    
    // print to console 
    println!("Number: {}", 1);
    
    // Basic Formatting
    println!("{} is from  {}", "Brad", "Earth");

    // positional arguments
    println!("{0} is from  {1} and {0} likes {2} ", "Brad", "Earth", "Choclate");

    // named arguments
    println!("{name} is from {planet} ", name="Brad", planet="Earth");

    //placeholder for debug
    println!("{:?}",(12, true, "hello"));

    // basic maths
    println!("10 + 10 = {}",10 + 10);
}