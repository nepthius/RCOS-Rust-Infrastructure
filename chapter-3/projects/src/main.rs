use std::io;
use std::collections::HashMap;

fn temp_conv(num:i32, temp:String)->f64{
    if temp=="f"{
        return ((num as f64) -32.0)*5.0/9.0
    }else{
        return ((num as f64)*9.0)/5.0 + 32.0
    }
}



fn main() {
    //This is the rust temp conversion block commenting to get rid of warning stuff in compiler
    /*
    println!("Enter F for Fahrenheit or C for Celsius");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Issue with reading");
    let mut temp = temp.trim().to_lowercase();

    println!("Enter a temp number you want to convert!");
    let mut num = String::new();


    io::stdin().read_line(&mut num).expect("Issue with number");
    let mut num: i32 = num.trim().parse().expect("Cannot convert temp");
    let result = temp_conv(num, temp);
    println!("Converted temp: {result}");
    */


    //fibbonaci, kinda works but don't know how to do memoization yet
    /*
    let mut fib_memo: HashMap<i32, i32> = HashMap::new();

    fn fib_gen(num:i32)->i32{
        if num <= 0{
            return 0
        }
    
        else if num==1 || num==2{
            return 1
        }

        if !fib_memo.contains_key(num){
            fib_memo.insert(num, fib_gen(num-1) + fib_gen(num-2));
        }

        return fib_memo.get(num)
    }
    

    let ret = fib_gen(4);
    println!("fib val: {ret}")
    */

    

}
