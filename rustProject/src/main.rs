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
    let _X: i32 ={
        let price: i32 =5;
        let qnty: i32 =5;
        price * qnty
    };
    let sum:i32= add(3,4);
    println!("Total sales is {}", _X);
    println!("Adding two values {}", sum);
    let bmi:f64=bmi(85.1, 1.6256);
    println!("My BMI is: {}", bmi);
    //ownership
    let s1= String::from("RUST");
     let s2= s1;
    let len=calculate_legth(&s2);
    println!("The leng is : {}", len);

    //dropping value if no owner if out of scope
    let s1= String::from("RUST");
     let len=calculate_legth(&s1);
     println!("The length of {} is  {}", s1, len);

     //borrowing and referrences
     // immutable and mutable referrences
     let mut im : i32 =5;
     let  r : &mut i32 = &mut im;
     *r+=1;
      println!("value of r is {}", r);

}

// function/ variables should be written in snake case
fn human_id(name: &str, age: u32, height: f32){
    println!("My name is: {}, I am {} years old, and my height is {}", name, age,height);
}
fn add(a:i32, b:i32)->i32{
    a+b
}
fn bmi(weight: f64, height: f64 )->f64{
    weight / (height * height)
}

//ownership 
fn  calculate_legth(s: &String)->usize{
    s.len()
}


