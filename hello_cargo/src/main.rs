#[derive(Debug)]
enum Usstate
{
    Alabama,
    Alaska
}
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(Usstate)
}
fn value_in_cents(coin:Coin)->u8{
    match coin{
        Coin::Penny=>1,
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin::Quarter(state)=>{
            println!("State quarter from {:?}!",state);
            25
        },
    }
 }
fn main(){
let x =value_in_cents(Coin::Quarter(Usstate::Alaska));
println!("{}",x);
}