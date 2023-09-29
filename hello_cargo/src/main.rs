use bookshelf::*;


fn main() {
    let mut book_1 = Book::new(String::from("Diary of a Wimpy Kid"),String::from("Jeff Kinney"),244);
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

    book_1.modify_title(String::from("DaVinci Code"));
    println!("Modified Book: {:#?}", book_1);

    let title = book_1.get_title();
    println!("Title: {}", title);

    // Function that borrows and modifies the title
   
}
