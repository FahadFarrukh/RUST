
fn numbers(num1:u32)->String{
    if num1%2==0{
        return String::from("Even");
    }
    else{
        return String::from("Odd");
    }
 
}


fn main(){
let num=6;

    let answer=numbers(num);
    println!("The number {} is {}",num,answer);

    let mut x=1;
    let mut y=0;
    
    while x<=10{
        y=y+x;
        x=x+1;
    }
    println!("{}",y);
    
// Slice 
    let string1 = "abcde";
    let string2 = "helloooo";

    let first_char1 = &string1[0..1]; 
    let last_char1 = &string1[string1.len() - 1..]; 

    let first_char2 = &string2[0..1]; 
    let last_char2 = &string2[string2.len() - 1..]; 

    println!("String 1: {}", string1);
    println!("First character of String 1: {}", first_char1);
    println!("Last character of String 1: {}", last_char1);

    println!("String 2: {}", string2);
    println!("First character of String 2: {}", first_char2);
    println!("Last character of String 2: {}", last_char2);





// longest



    fn find_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() >= s2.len() {
            s1
        } else {
            s2
        }
    }
    
    fn main() {
        let string1 = "abcde";
        let string2 = "helloooo";
    
        let longest = find_longest(string1, string2);
        
        println!("The longest string is: {}", longest);
    }
    



//greatest number in array
let numbers = [5, 8, 2, 10, 30];
let mut x = 0;
let length = numbers.len();
let mut greatest: i32 = numbers[0]; // Initialize with the first element

while x < length {
    if numbers[x] > greatest {
        greatest = numbers[x]; // Update if the current element is greater
    }
    x = x + 1;
}

println!("{}", greatest);

















// fn reverse_string(input: &str) -> String {
//     let reversed: String = input.chars().rev().collect();
//     reversed
// }

// fn main() {
//     let input_string = "Hello, World!";
//     let reversed_string = reverse_string(input_string);
//     println!("Original: {}", input_string);
//     println!("Reversed: {}", reversed_string);
// }
    







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




}



// #[derive(Debug)]
// struct Rectangle {
// width: u32,
// height: u32,
// }
// fn main() {
// let scale = 2;
// let rect1 = Rectangle {
// width: dbg!(30 * scale),
// height: 50,
// };
// dbg!(&rect1);
// }






// struct Rectangle {
//     width: u32,
//     height: u32,
//     }
//     impl Rectangle {
//     fn new(width:u32,height:u32) -> Self {
//     Self{
//         width,
//         height
//     }
//     }
//     }
//     fn main() {
//     let rect = Rectangle::new(10,20);
//     println!(
//     "The width is {} and the height is {}.",rect.width,rect.height);
//     }
    



 
// struct Circle {
//     radius:f64,
//     }
//     impl Circle {
//     fn new(radius:f64) -> Self {
//         Self{radius}
//     }
//     fn area(&self){
        
//         &self.radius*&self.radius*3.164;
     
//     }
    
//     }
//     fn main() {
//     let circle = Circle::new(20.0);
//     let area=Circle::area(&circle);
//     println!(
//     "The radius is {}.",circle.radius);
//     println!(
//         "The area is {:?}.",&area);
//     }
       

// enum IpAddrKind {
//     V4,
//     V6,
// }
// struct IpAddr{
//     kind: IpAddrKind,
//     address:String,
// }

// fn route(ip_kind:IpAddrKind){}
// fn main(){
//     let four=IpAddrKind::V4;
//     let six=IpAddrKind::V6;

//     route(four);
//     route(six);

// }



// #[derive(Debug)]
// enum Usstate
// {
//     Alabama,
//     Alaska
// }
// enum Coin{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(Usstate)
// }
// fn value_in_cents(coin:Coin)->u8{
//     match coin{
//         Coin::Penny=>1,
//         Coin::Nickel=>5,
//         Coin::Dime=>10,
//         Coin::Quarter(state)=>{
//             println!("State quarter from {:?}!",state);
//             25
//         },
//     }
//  }
// fn main(){
// let x =value_in_cents(Coin::Quarter(Usstate::Alaska));
// println!("{}",x);
// }


// #![allow(unused_variables)]

// fn main(){
//     let mut v = Vec::new();

//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);

//   //  print!("{:#?}",v);
//   dbg!(v);
// }