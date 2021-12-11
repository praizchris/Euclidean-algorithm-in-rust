fn euclidean(mut num1:i32,mut num2:i32) -> i32{
    let mut reminder : i32;
    while num1 % num2 > 0 {
        reminder = num1 % num2;
        num1 = num2;
        num2 = reminder;
    }
    return num2;    
}

fn main() {
   let result = euclidean(270,192);
   println!("{}", result);
} 
