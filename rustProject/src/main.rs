fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
    //i32, i64 can be negative or positive
    //u32, u64 positive int
    //f32,f64 float
    let pi:f64=3.1415;
    println!("Value of pi: {}",pi);
    let is_snowing: bool=true;
    println!("Is it snowing? {}", is_snowing);
    // charater type 
    let letter: char ='a';
    println!("First letter: {}", letter);
    // arrays
    let number : [i32;5]=[1,2,3,4,5];
    println!("Array numbers: {:?}", number);
    //mixed array
    let mixed : [&str; 3]=["John Doe","Maria","Janice"];
    
    println!("Midex Array: {:?}", mixed);
    // display per index
     println!("Midex Array: {}", mixed[0]);

     //tuples mixed
     let human=("Alice",30,false, [1,2,3,4,5]);
       println!("Tuple values: {:?}", human);
     //to string 
      let human2: (String,i32,bool)=("Alice".to_string(),30,false);
     println!("Tuple values with defined types: {:?}", human2);
    // slices
     let number_slices:&[i32]=&[1,2,3,4,5];
     println!("Slices: {:?}", number_slices);
     //string slice
      let string_slices:&[&str]=&["Lion","MOnkey",];
     println!("Slices: {:?}", string_slices);

     // //string slice to string
      let string_slices:&[&String]=&[&"Sen".to_string(),&"Honey".to_string(),];
     println!("Slices: {:?}", string_slices);

     //string growable, mutable, owned, string type
     let mut stone_cold: String = String :: from("Hell");
     println!("Stone cold: {}", stone_cold);
     stone_cold.push_str("Yeah");
      println!("Stone cold: {}", stone_cold);
     // b-&str(string slice)
     let string: String = String :: from("Hello, World!");
     let slice : &str = &string; 

     // getting specific value
     let slice : &str = &string[0..5]; 
    println!("Slice value: {}", slice);
    human_id("Jaime", 33, 184.67);
}

// function/ variables should be written in snake case
fn human_id(name: &str, age: u32, height: f32){
    println!("My name is: {}, I am {} years old, and my height is {}", name, age,height);
}


