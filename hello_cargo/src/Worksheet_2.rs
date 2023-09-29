//1. #![allow(unused_variables)]
// #[derive(Debug)]


// struct Person {
// name: String,
// age: u32,
// }
// impl Person {
//     fn new(name: String,age: u32,) -> Self {
//     Self{
//         name,
//         age
//         }
//     }
// }

// fn main() {
//     let person1 = Person::new(String::from("Fahad"),22);
//     println!("{:#?}",person1);
 
// }



 //2. Define the Color enum
// enum Color {
//     Red,
//     Green,
//     Blue,
// }

// // Define a function to convert Color to RGB
// fn color_to_rgb(color: Color) -> (u8, u8, u8) {
//     match color {
//         Color::Red => (255, 0, 0),
//         Color::Green => (0, 255, 0),
//         Color::Blue => (0, 0, 255),
//     }
// }

// fn main() {
//     let red_rgb = color_to_rgb(Color::Red);
//     let green_rgb = color_to_rgb(Color::Green);
//     let blue_rgb = color_to_rgb(Color::Blue);

//     println!("Red: {:?}", red_rgb);
//     println!("Green: {:?}", green_rgb);
//     println!("Blue: {:?}", blue_rgb);
// }



//3.



// fn add_tuple(num_1:u32,num_2:u32) -> u32 {
//     num_1+num_2
//   }
  
//   fn main() {
   
//      let function = add_tuple(10,5);
//      println!("{}",function);
//   }
  


//4.
// #[derive(Debug)]
// enum Option {
//     Text(String),
//     Number(u32),
// }

// fn print(option: Option) {
//     match option {
//         Option::Text(text) => println!("Text: {}", text),
//         Option::Number(num) => println!("Number: {}", num),
//     }
// }

// fn main() {
//     let text_option = Option::Text(String::from("Hello, Rust!"));
//     let number_option = Option::Number(42);

//     print(text_option);
//     print(number_option);
// }

