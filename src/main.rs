use std::io;

struct Position {
    //it helps to store the position of the projectile and the interceptor as well as the velocity of the projectile
    x: f64,
    y: f64,
    z: f64,
}

const INTERCEPTOR: [u16; 3] = [80, 150, 250]; //it is the set of three interceptors speed to be checked. if they can catch the projectile or not

fn main() {
    //getting the position fo the interceptiee
    println!("enter values for the first position");
    let pos1 = intput();
    println!("enter values for the second position");
    let pos2 = intput();
    let velocity = Position {
        x: pos2.x - pos1.x,
        y: pos2.y - pos1.y,
        z: pos2.z - pos1.z,
    };

    for interceptor in INTERCEPTOR {
        //checking if the interceptor can catch the projectile or not
        if check(&velocity, &pos1, &interceptor) {
            let time = time_to_intercept(&velocity, &pos1, &interceptor).unwrap();
            let pos_t = Position {
                x: pos2.x + velocity.x * (time - 1.0),
                y: pos2.y + velocity.y * (time - 1.0),
                z: pos2.z + velocity.z * (time - 1.0),
            };
            let (azimuth, elevation) = get_angles(&pos_t);
            println!(
                "The projectile can be intercepted with a speed of {} at time {} seconds, with an azimuth of {} degrees and an elevation of {} degrees",
                interceptor,
                time,
                azimuth.to_degrees(),
                elevation.to_degrees()
            );
        } else {
            println!(
                "The projectile cannot be intercepted with a speed of {}",
                interceptor
            );
            continue;
        }
    }
}

fn get_angles(pos: &Position) -> (f64, f64) {
    let azimuth = pos.y.atan2(pos.x);
    let elevation = (pos.z / (pos.x.powi(2) + pos.y.powi(2) + pos.z.powi(2)).sqrt())
        .clamp(-1.0, 1.0)
        .asin();
    (azimuth, elevation)
}

fn check(v: &Position, pos1: &Position, speed: &u16) -> bool {//returns yes if the interceptor is getting a valid time to intercept the projectile
    time_to_intercept(v, pos1, speed).is_some()
}

fn time_to_intercept(v: &Position, pos1: &Position, speed: &u16) -> Option<f64> {//sends back the interception minimum time or if not possible
    let a = v.x * v.x + v.y * v.y + v.z * v.z - (*speed as f64 * *speed as f64);
    let b = 2.0 * (pos1.x * v.x + pos1.y * v.y + pos1.z * v.z);
    let c = pos1.x * pos1.x + pos1.y * pos1.y + pos1.z * pos1.z;

    if a.abs() < 1e-9 {
        if b.abs() < 1e-9 {
            return None;
        }
        let t = -c / b;
        return if t > 0.0 { Some(t) } else { None };
    }

    let d = b * b - 4.0 * a * c;
    if d < 0.0 {
        return None;
    }
    let t1 = (-b + d.sqrt()) / (2.0 * a);
    let t2 = (-b - d.sqrt()) / (2.0 * a);
    if t1 >= 0.0 && t2 >= 0.0 {
        return Some(t1.min(t2));
    }
    if t1 >= 0.0 {
        return Some(t1);
    }
    if t2 >= 0.0 {
        return Some(t2);
    }
    None
}

//makes it easier to enter the new data
fn intput() -> Position {
    println!("enter the x value");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");
    let input1: f64 = input1.trim().parse().expect("Please type a number!");
    println!("enter the y value");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");
    let input2: f64 = input2.trim().parse().expect("Please type a number!");
    println!("enter the z value");
    let mut input3 = String::new();
    io::stdin()
        .read_line(&mut input3)
        .expect("Failed to read line");
    let input3: f64 = input3.trim().parse().expect("Please type a number!");
    Position {
        x: input1,
        y: input2,
        z: input3,
    }
}
