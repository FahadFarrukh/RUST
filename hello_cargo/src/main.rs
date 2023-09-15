// fn factorial(x: u32) -> u32 {
//     if x == 0 {
//         1
//     } else {
//         x * factorial(x - 1)
//     }
// }


// fn sum(x: u32, y: u32) -> u32 {
//   x + y



// }
// }
fn main() {
    // const MAX_POINTS: u32 = 100_000;

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // const PI: f64 = 3.14159;

// let y=sum(5,4);
// println!("{y}");

    // let y = sum(7);
    // println!("{y}");

    // let condition = true;
    // let number = if condition { 5 } else { 6 };

    // println!("{number}");

    // // let x: i32 = 5;
    // // let mut y: i32 = 10;
    // let radius: f64 = 5.0;
    // let mut area: f64 = 0.0;

    // let mut y = 0;

    // while y < 100 {
    //     y = y + 1;
    //     if y % 3 == 0 {
    //         println!("Fizz");
    //     }
    //     if y % 5 == 0 {
    //         println!("Buzz");
    //     }
    //     if y % 3 == 0 && y % 5 == 0 {
    //         println!("FizzBuzz");
    //     } else {
    //         println!("{y}");
    //     }
    // }

    // area = PI * radius * radius;

    // println!("The value of y is : {area}");
    // let h = sum(5, 6);
    // println!("{h}")
    // y = 15;
    // println!("The value of y is : {y}");
    // println!("{MAX_POINTS}");
    // println!("{THREE_HOURS_IN_SECONDS}");

    // let mut x= 0;
    // x=x+1;
    // {
    //     let x = x*2;
    //     println!("The value of x is : {x}");
    // }

    // println!("The value of x is : {x}");

    // let tup = (500, 6.0, 4);
    // let (x, y, z) = tup;
    // {
    //     println!("{y}");
    // }
    // let s1 = String::from("Hello");
    // let s2=s1;
    // println!("{}",s2);


//ownership
    let s1 = String::from("Hello");
    takes_ownership(s1);


}
fn takes_ownership(str:String){
    println!("{}",str)
}