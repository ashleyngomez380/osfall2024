/// This is the constant

const FARENHEIT : f64 = 32.0;

fn farenheit_to_celsius(f: f64) -> f64{

    let result = (f - FARENHEIT) * 5.0 / 9.0;
    return result;

}

fn celsius_to_fahrenheit(c: f64) -> f64{
    
    let result = (c * 9.0 / 5.0) + FARENHEIT;
    return result;

}


fn main() {
    // Declaring
    let mut temp = 100.00;   
    // For loop
    for _ in 0 .. 5 { // FYI _ is called a step
        // Converting 
        let celtemp = farenheit_to_celsius(temp);
        println!("{:.1} F becomes {:.2} C",temp ,celtemp);
        // Increment by One
        temp += 1.0;
    }
}
