


fn main() {
    // Простое условие
    let number = 7;
    if number < 5 {
        println!("условие должно быть истинным");
    } else {
        println!("условие было ложным");
    }

    let number = 3;
    if number !=0 {
        println!("число должно быть отличным от нуля");
    }
    
    // Обработка нескольких условий
    let number = 6;
    if number % 4  == 0 {
        println!("Число делится на 4");
    } else if number % 3 == 0 {
        println!("Число делится на 3");
    } else if number % 2 == 0 {
        println!("Число делится на 2");
    } else {
        println!("число не делится  на 4, 3 и 2");
    }
    
    //Использование if в let инструкции
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("Значение числа равно {}", number);
}
