#![allow(unused_variables)]

fn main(){
    let mut v = vec![100,32,57];

   let first=&v[0];

   for i in &mut v{
*i +=50;
println!("{}",i);
   }
}