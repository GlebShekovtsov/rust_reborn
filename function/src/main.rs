
fn main() {
    // another_function(5, 6);
    // let x = 5;

    // let y ={
    //     let x = 3;
    //     x + 1
    // };
    // println!("Значение y равно {}", y);
    let x  = five();
    println!("Значение x равно {}", x);

    let y = plus_one(5);
    println!("Значение x равно {}", y);
}

fn five() -> i32{
    5
}
fn plus_one(y:i32) -> i32 {
    y+1
}

// fn another_function(x: i32, y:i32) {
//     println!("Значение x равно {}", x);
//     println!("Значение y равно {}", y);
// }