// Statically Typed Language. If not assigned it will be infered
// Primitives : Integers, Floats (f32, f64), Tuples, Characters (char), Boolean (bool), Arrays

pub fn run(){
    // default i32;
    let _x = 1; 

    // default f64;
    let _y = 2.5; 

    // explicitly ;
    let _z: i64 = 45545445;
    
    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    // Find max size
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;
    println!("OP: {:?}", (_x, _y, _z, is_active));

    // Boolean From Expression
    let is_active_exp: bool = 10 > 1;
    let chara: char = 'a';
    let face = '\u{1F600}';
    println!("OP: {:?}", (_x, _y, _z, is_active,is_active_exp, chara, face));

    
    
}