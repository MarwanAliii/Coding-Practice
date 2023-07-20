// Problem 1
pub fn problem1() {
    let x: i32 = 5; // uninitialized but using, ERROR !
    let y: i32; // uninitialized but also unusing, only warning
    println!("{} is equal to 5", x);
}

// Problem 2
pub fn problem2() {
    let mut x = 1;
    x += 2;

    println!("{} is equal to 3", x);
}

// Problem 3
pub fn problem3() {
    let x: i32 = 10;
    let y: i32 = 20;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}

// Problem 4
pub fn problem4a() {
    let x = define_x1();
    println!("{}, world", x);
}
pub fn define_x1() -> String {
    let x = "hello".to_string();
    x
}

pub fn problem4b() {
    let x = define_x2();
    println!("{:?}, world", x);
}

pub fn define_x2() -> &'static str {
    let x = "hello";
    x
}

// Problem 5
pub fn problem5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

// Problem 6
pub fn problem6() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}

// Problem 7
pub fn problem7() {
    let _x = 1;
    // or could have used #[allow(unused_variables)]
}

// Problem 8
pub fn problem8() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}

// Problem 9
pub fn problem9() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // fill the blank to make the code work
    assert_eq!([x, y], [3, 2]);
}