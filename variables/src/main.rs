//const MAX_POINTS: u32 = 100_000; - константа
fn main() {
    // Затирание
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    let spaces = " ";
    let spaces = spaces.len();
    println!("Значение х равно {}", x);
    let guess: u32 = "42".parse().expect("Не является числом!");
    
    // Операции с переменными
    let sum = 5 + 10; //
    let differende = 95.5 - 4.3; 
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("Значение равно {}", sum);
    println!("Значение равно {}", differende);
    println!("Значение равно {}", product);
    println!("Значение равно {}", quotient);
    println!("Значение равно {}", remainder);

    // Булев тип
    let t = true;
    let f : bool = false;

    // Символьный тип
    let c = 'z';
    let z = 'ƶ';
    let cat = '🐈';

    // Кортежный тип
    let zatup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = zatup;
    println!(" Значение x равно {} \n Значение y равно {} \n Значение z равно {}", x,y,z);
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!(" Значение One равно {}", one);

    // Массив
    let a = [1,2,3,4,5];
    let months = ["Январь", "Февраль", "Март", "Апрель", "Май", "Июнь", "Июль", "Август", "Сентябрь", "Октябрь", "Ноябрь", "Декабрь"];
    let a = [3; 5];
    
    //Доступ к элементам массива
    let a = [1,2,3,4,5];
    let first = a[0];
    let second = a[1];
    
    //Недействительный доступ к элементу массива
    // let a = [1,2,3,4,5];
    // let index = 10;
    // let element = a[index];
    // println!("Значение элемента равно {}", element);

    
}
