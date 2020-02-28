use rand::Rng;

fn main() {
    let tries = 1000;
    let ncoupons = 10;
    let mut boxeslist: Vec<u32> = Vec::new();
    
    let mut expected: f32 = 0.0;
    let mut mean: f32 = 0.0;
    
    for i in 1..ncoupons+1 {
        expected += 1.0 / (i as f32);
    }
    
    expected *= ncoupons as f32;
    
    let mut rng = rand::thread_rng();
    
    for _ in 0..tries {
        let mut boxes: u32 = 0;
        let mut coupons: Vec<u32> = Vec::new();
        
        for n in 0..ncoupons {
            coupons.push(n);
        }
        
        while coupons.len() != 0 
        {
            boxes += 1;
            let coupon = rng.gen_range(0, ncoupons);
            
            match coupons.binary_search(&coupon) {
              Ok(index) => coupons.remove(index),
              Err(_) => 0
            };
        }
        
        boxeslist.push(boxes);
    }
    
    for b in boxeslist.iter() 
    {
        mean += *b as f32;
    }
    
    print!("La cantidad media de intentos fue {}. \n", ((mean as f32) / (tries as f32)) as u32);
    print!("La cantidad de intentos esperada era {}.\n", expected as u32);
}

