
fn recursive(num:u32)->u32{
    let mut z=0;
    let mut y=0;
    let mut sum=10;
    
while z<num{
    y=sum*10;
    sum=sum+y;
    y=y+y;
    z=z+1;
    
}
return sum;
}


fn main() {
    let number=5;
    let x=recursive(number);
println!{"{}",x};
}
