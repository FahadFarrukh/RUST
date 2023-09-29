struct Date {
    day: u32,
    month: u32,
    year: u32,
}
 
impl Date {
    fn new(day: u32, month: u32, year: u32) -> Date {
        Date { day, month, year }
    }
    fn day(&self) -> u32 {
        self.day
    }
    fn month(&self) -> u32 {
        self.month
    } 
    fn year(&self) -> u32 {
        self.year
    }
    fn isLeap(&self) -> bool {
        self.year % 4 == 0
    }
    fn isleap(&self) -> bool {
        self.year % 4 == 0
    }
    fn add_day(day:u32)->u32{
        day+1
    }

}


    fn main() {
        let mut date = Date::new(23, 9, 2023);
    
        println!("Date: {}-{}-{}", date.year, date.month, date.day);
    
        if date.isleap() {
            println!("{} is leap year.", date.year);
        } else {
            println!("{} is not leap year.", date.year);
        }

        let x=Date::add_day(23);

        println!("Next Date is {}",x);
    
       
    }



    // enum Coin{
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter
    
    // }
    // fn value_in_cents(coin:Coin)->u8{
    //     match coin{
    //         Coin::Penny=>1,
    //         Coin::Nickel=>5,
    //         Coin::Dime=>10,
    //         Coin::Quarter=>15,
    //     }
    //  }
    // fn main(){
    // let x =value_in_cents(Coin::Dime);
    // println!("{}",x);
    // }


//     #![allow(unused_variables)]

// fn main(){
//     let mut v = vec![100,32,57];

//    let first=&v[0];

//    for i in &mut v{
// *i +=50;
// println!("{}",i);
//    }
// }