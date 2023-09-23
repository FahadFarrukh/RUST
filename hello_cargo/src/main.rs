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