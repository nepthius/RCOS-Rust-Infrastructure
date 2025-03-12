fn main() {
    let a = String::from("Hello");
    //print!("{a}");
    let val = fword(&a);
    print!("{val}");

} 

fn fword(s: &String) -> String{
    let mut ret = String::from("");
    for (i, c) in s.chars().enumerate(){
        if c == ' '{
            break
        }
        ret.push(c)
    }
    ret
}