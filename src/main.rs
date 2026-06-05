use std::io;

struct Position{
    x: f64,
    y: f64,
    z: f64,
}

const INTERCEPTOR: [u16; 3] = [80,150,250];

fn main() {
    //getting the position fo the interceptiee
    println!("enter values for the first position");
    let pos1 = intput();
    println!("enter values for the second position");
    let pos2 = intput();
    let velocity = Position{
        x: pos2.x - pos1.x,
        y: pos2.y - pos1.y,
        z: pos2.z - pos1.z,
    };
    for interceptor in INTERCEPTOR{
        if check(&velocity, &pos1, &interceptor){
            println!("The projectile can be intercepted with a speed of {}", interceptor);
        }
        else{
            println!("The projectile cannot be intercepted with a speed of {}", interceptor);
            continue;
        }
    }   
}

fn check(v: &Position, pos1: &Position, speed: &u16) -> bool {
    let a = v.x * v.x + v.y * v.y + v.z * v.z - (*speed as f64 * *speed as f64);
    let b = 2.0 * (pos1.x * v.x + pos1.y * v.y + pos1.z * v.z);
    let c = pos1.x * pos1.x + pos1.y * pos1.y + pos1.z * pos1.z;

    let d = b*b - 4.0*a*c;
    if d < 0.0 {
        return false;
    }
    let t1 = (-b + d.sqrt()) / (2.0 * a);
    let t2 = (-b - d.sqrt()) / (2.0 * a);
    if t1 >= 0.0 || t2 >= 0.0 {
        return true;
    }
    false
}
//makes it easier to enter the new data
fn intput()-> Position{
    println!("enter the x value");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let input1: f64 = input1.trim().parse().expect("Please type a number!");
    println!("enter the y value");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let input2: f64 = input2.trim().parse().expect("Please type a number!");
    println!("enter the z value");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read line");
    let input3: f64 = input3.trim().parse().expect("Please type a number!");
    Position{x: input1, y: input2, z: input3}
}