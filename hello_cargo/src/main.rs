
#[derive(Debug)]
enum  Status{
    Active,
    Inactive,
    Suspended
}


#[derive(Debug)]
struct Book {
title: String,
author: String,
pages: u32

}
impl Book {
    fn new(title: String,author: String,pages: u32) -> Self {
    Self{
        title,
        author,
        pages
        }
    }
    fn title(&self) -> &String {
        &self.title
    }
    fn author(&self) -> &String {
        &self.author
    }
}
fn book_status(title:String,status:Status)->(String,Status){

    match status {
        Status::Active=>(title,status),
        Status::Inactive=>(title,status),
        Status::Suspended=>(title,status),

    }

}

fn print_option(option: Option<i32>) {
    match option {
        Some(_) => println!("Has a value"),
        None => println!("No value"),
    }
}



fn main() {
    let book_1 = Book::new(String::from("Diary of a Wimpy Kid"),String::from("Jeff Kinney"),244);
    println!("{:#?}",book_1);
    println!("\nTitle: {}", book_1.title());

    let status=book_status(String::from("Diary of a Wimpy Kid"), Status::Active);
    println!("\nStatus: {:#?}", status); 


    let author=book_1.author();

    let some_i32 = Some(10);
    let none_i32: Option<i32> = None;

    print_option(some_i32);
    print_option(none_i32);

    println!("\nTitle: {}", book_1.author());
}
