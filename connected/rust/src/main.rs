use rand::Rng;

fn main() {
    let tries: u64 = 40;
    let nodes: u64 = 2000;
    let prob: f64 = 0.0070;
    let mut conn: u64 = 0;
    
    let mut rng = rand::thread_rng();
    
    for _ in 0..tries {
    
        let mut matrix = vec![Vec::new(); nodes as usize];
        let mut openl = vec![0];
        let mut lnodes = Vec::new();
        
        for r in 0..nodes {
            lnodes.push(r);
            for c in r+1..nodes {
                let x = rng.gen::<f64>();
               
                if x > ((1 as f64) - prob) {
                    matrix[r as usize].push(c);
                    matrix[c as usize].push(r);
                }
            }
        }
    /*
        for (_i, row) in matrix.iter_mut().enumerate() {
            for (_j, col) in row.iter_mut().enumerate() {
                print!("{} ", col);
            }
            println!();
        }
    */
        while openl.len() != 0 && lnodes.len() != 0 {
            let node = openl.remove(0);
            
            match lnodes.binary_search(&node) {
                Ok(index) => {
                    lnodes.remove(index); 
                    let openn = &matrix[node as usize];
                    openl.extend(openn.iter().cloned());
                }
                Err(_) => ()
            };
            
            if lnodes.len() == 0 {
                conn = conn + 1;
            }
        }
    }
    
    print!("Connected: {} %. \n", ((conn as f64) * 100 as f64 / (tries as f64)) as u64);
}

