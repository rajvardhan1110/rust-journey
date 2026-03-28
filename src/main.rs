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



    //iterator


    // iter()
    // Note: Read-only iterator (borrows &T, cannot modify, vector stays usable)
    let v1 = vec![1, 2, 3];
    for x in v1.iter() {
        println!("iter: {}", x);
    }
    println!("v1 still usable: {:?}", v1);


    // iter_mut()
    // Note: Mutable iterator (borrows &mut T, can modify, vector stays usable)
    let mut v2 = vec![1, 2, 3];
    for x in v2.iter_mut() {
        *x += 10;
    }
    println!("iter_mut modified v2: {:?}", v2);


    // into_iter()
    // Note: Ownership iterator (moves values, vector becomes unusable)
    let v3 = vec![1, 2, 3];
    for x in v3.into_iter() {
        println!("into_iter: {}", x);
    }
    // println!("{:?}", v3); // ❌ ERROR (v3 moved)


    // Consumer Adapter
    // Note: Consumes iterator and gives final result (vector safe if using iter())
    let v4 = vec![1, 2, 3];
    let sum: i32 = v4.iter().sum();
    println!("sum: {}", sum);
    println!("v4 still usable: {:?}", v4);


    // Iterator Adapter
    // Note: Lazy transformation (runs only when consumer like collect() is used)
    let v5 = vec![1, 2, 3, 4];
    let res: Vec<i32> = v5.iter()
        .map(|x| x * 2)      // adapter
        .filter(|x| x > &4)  // adapter
        .collect();          // consumer
    println!("result: {:?}", res);

    //string and slice
    //Slice from String
    // Note: &str can point inside String
    let s3 = String::from("hello world");
    let slice:&str = &s3[0..5];
    println!("slice: {}", slice);

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