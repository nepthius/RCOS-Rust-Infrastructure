fn dame() {
    let mut x = 8;
    println!("value of x: {x}");
    x = 10;
    println!("value of x: {x}");
    const H: u32 = 10*10;
    println!("value of H: {H}");
    another_one(H);
    let five: i32 = five(3);
    println!("five:{five}");
}

fn another_one(x: u32){
    println!("Waterboarded {x}")
}

fn five(mut x:i32)->i32{
    x=x-1;
    x+1
}

fn main() {
    let num = 3;
    if num < 10{
        println!("LESS!");
    } else{
        println!("MORE! OR EQUAL");
    }

    let mut num = if num%3==0 {10} else {8};
    println!("Number: {num}");

    while num > 0{
        num-=1;
    }
    println!("{num}");
}

