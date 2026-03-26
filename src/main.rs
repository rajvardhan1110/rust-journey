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