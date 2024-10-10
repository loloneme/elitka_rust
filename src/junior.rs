fn divide_numbers(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        return Err("Division by zero is not allowed");
    } else if b < 0.0 || a < 0.0{
        return Err("One of the numbers is negative");
    }
    Ok(a / b)
}

fn junior(){
    let mut x = 10.0;
    let mut y = 0.0;

    let result = divide_numbers(x, y);
    
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("In expression {:?} / {:?} An error occurred: {}", x, y, e),
    }


    y = 5.0;
    let result_2 = divide_numbers(x, y);
    match result_2 {
        Ok(value) => println!("Result 2: {}", value),
        Err(e) => println!("In expression {:?} / {:?} An error occurred: {}", x, y, e),
    }

    x = -6.0;
    let result_3 = divide_numbers(x, y);
    match result_3 {
        Ok(value) => println!("Result 3: {}", value),
        Err(e) => println!("In expression {:?} / {:?} An error occurred: {}", x, y, e),
    }
}

fn main() {
    junior();
}