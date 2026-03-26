use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");


    //variables
    //numbers
    let x : i32 = -5;
    let y : u32 = 1000;
    let z : f32 = 10.05;


    println!("x : {}",x);
    println!("y : {}",y);
    println!("z : {}",z);

    //boolean
    let is_male:bool = true;

    if is_male{
        println!("you are a male");
    }else{
        println!("you are not a male");
    }

    //string 
    let greeting = String::from("hello world");
    println!("{}", greeting);

    //conditional 

    let number = 10;

    if number > 5 {
        println!("Number is greater than 5");
    } else {
        println!("Number is 5 or less");
    }


    //loop

    for i in 1..5 {
        print!("{} ", i);
    }
    println!();

    let mut i = 1;

    while i <= 3 {
        print!("{} ", i);
        i += 1;
    }
    println!();


    //functions
    let result = add(5, 3);
    println!("5 + 3 = {}", result);


    //struct & impl
    let p = Person { name: String::from("Raj") };
    p.greet();


    //enum
    move_player(Direction::Up);
    move_player(Direction::Left);

    // error handling using Result enum

    // enum Result<T, E> {
    // Ok(T),
    // Err(E),

    let result = divide(10, 2);
    
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
    
    let result2 = divide(10, 0);
    
    match result2 {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    //Option enum
    // enum Option<T> {
    // Some(T),  // value exists
    // None,     // no value

    let numbers = [10, 20, 30];
    
    match find_number(&numbers, 20) {
        Some(index) => println!("Found at index {}", index),
        None => println!("Not found"),
    }
    
    match find_number(&numbers, 50) {
        Some(index) => println!("Found at index {}", index),
        None => println!("Not found"),
    }


    //collection

    //vector

    let mut v = vec![1,2,3,4,5];

    println!("Original vector: {:?}", v);

    println!("\nImmutable iteration:");
    for val in &v {
        println!("{}", val);
    }

    for val in &mut v {
        *val *= 2;  // double each element
    }

    println!("After mutable iteration vector: {:?}", v);

                // Push and Pop
    v.push(100);
    println!("\nAfter push: {:?}", v);

    v.pop();
    println!("After pop: {:?}", v);
            
            //particular index
    match v.get(1) {
        Some(val) => println!("{}", val),
        None => println!("Index out of bounds"),
    }

    //Hashmap
        //import required -> use std::collections::HashMap;
    let mut map = HashMap::new();

    map.insert("apple", 10);
    map.insert("banana", 20);

    println!("{:?}", map);

    

}

fn find_number(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &num) in arr.iter().enumerate() {
        if num == target {
            return Some(i); // found
        }
    }
    None // not found
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_player(dir: Direction) {
    match dir {
        Direction::Up => println!("Move up"),
        Direction::Down => println!("Move down"),
        Direction::Left => println!("Move left"),
        Direction::Right => println!("Move right"),
    }
}

struct Person {
    name: String,
}

impl Person {
    fn greet(&self) {
        println!("Hello {}", self.name);
    }
}

fn add(a: i32, b: i32) -> i32 {
   return a + b  
}