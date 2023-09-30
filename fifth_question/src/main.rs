fn main() {
    let array=['l','o','l'];
    let mut a=0;
    let mut y=3;
    while y<=0{
        let z = array[y];
        let f = array[a];
        if z==f{
            println!("Palindrome");
        }
        else{
            println!("Not Palindrome");
        }
        a=a+1;
        y=y-1;
    }
   
}
