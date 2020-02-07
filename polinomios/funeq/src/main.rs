use std::io;
use rand::Rng;

fn main() {
    
    let mut coefs: Vec<f64> 
       = vec![-  98896140000.0, - 285056496000.0, - 247671152550.0,
              -  23271127065.0,    48717850707.0,     8138130777.0,
              -   2628372129.0, -    242157818.0,       48345358.0,
                     1851858.0, -       335880.0, -         5277.0,
                         975.0,              5.0, -            1.0];
    let mut roots: Vec<f64> 
       = vec![   1.0, - 15.0, -  6.0,    3.0,    7.0,   17.0, - 12.0,
              - 19.0,    1.0,    1.0, -  3.0,   20.0,   15.0, - 15.0];
    let mut sign: f64 = -1.0;
    
    let mut done = 'n';
    
    println!("Valores (1) predeterminados o (2) propios: ");
    let custom = char_input();
    
    println!("Número de ejecuciones del algoritmo: ");
    let tries = unsigned_input();
    
    println!("Factor: ");
    let factor: f64 = floating_input();
 
    println!("Cantidad de digitos de redondeo: ");
    let digits = signed_input() as u32;

    if custom == '2' {
        let mut power = 0;
    
        coefs = Vec::new();
        roots = Vec::new();
        
        while done == 'n' {
            println!("Inserta el coeficiente de x^{}: ", power);
            coefs.push(floating_input());
            println!("Has terminado? [s/n]");
            done = char_input();
            power = power + 1;
        }
        
        done = 'n';
        
        while done == 'n' {
            println!("X es negativa? [s/n] ");
            let neg = char_input();

            println!("Inserta la constante del binomio (x + c): ");
            let mut root = floating_input();
            
            if neg == 's' {
                sign = -sign;
                root = -root;
            }
            
            roots.push(root);
            println!("Has terminado? [s/n]");
            done = char_input();
        }
    }
    
    let mut gen = rand::thread_rng();
    let max = factor * roots.len() as f64;
    let mut equal = true;
    
    for _ in 0..tries {
        let x: f64 = roundp(gen.gen_range(-max, max), digits) as f64;
        
        let mut ra: f64 = 0.0;
        let mut rb: f64 = 1.0;
        
        let mut power: f64 = 0.0;
        
        for c in coefs.iter() {
            ra += *c * f64::powf(x, power);
            power += 1.0;
        }
        
        for r in roots.iter() {
            rb *= x + *r;
        }
        
        rb *= sign;
        
        ra = roundp(ra, digits);
        rb = roundp(rb, digits);
        
        println!("x = {}, ra = {}, rb = {}", x, ra, rb);
        
        if ra != rb {
            println!("Los polinomios son diferentes.");
            equal = false;
            break;
        }
    }
    
    if equal { println!("Los polinomios son iguales"); }
}

fn floating_input() -> f64 {
    let mut input = String::new();
    let mut number = 0.0;
    
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    
    let trimmed = input.trim();
    
    match trimmed.parse::<f64>() {
        Ok(i) => number = i,
        Err(..) => println!("Input {} is not an floating number.", trimmed),
    };
    
    number
}

fn unsigned_input() -> u32 {
    let mut input = String::new();
    let mut number = 0;
    
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    
    let trimmed = input.trim();
    
    match trimmed.parse::<u32>() {
        Ok(i) => number = i,
        Err(..) => println!("Input {} is not an unsigned integer.", trimmed),
    };
    
    number
}

fn signed_input() -> i64 {
    let mut input = String::new();
    let mut number = 0;
    
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    
    let trimmed = input.trim();
    
    match trimmed.parse::<i64>() {
        Ok(i) => number = i,
        Err(..) => println!("Input {} is not an integer.", trimmed),
    };
    
    number
}

fn char_input() -> char {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    
    let retval = input.trim().chars().next().unwrap();
    retval
}

fn roundp(input: f64, places: u32) -> f64 {
    let factor: f64 = f64::powf(10.0, places as f64);
    let mut temp: f64 = input * factor;
    temp = f64::round(temp);
    let output: f64 = temp / factor;
    
    output
}
