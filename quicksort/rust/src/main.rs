use rand::Rng;
use std::time::Instant;

static mut comparisons: i64 = 0;

fn swap (ilist: &mut Vec<i64>, i: i64, j: i64) {
    let tmp = ilist[i as usize];
    ilist[i as usize] = ilist[j as usize];
    ilist[j as usize] = tmp;
}

fn order(ilist: &mut Vec<i64>, start: i64, end: i64) -> i64 {
    let mut rng = rand::thread_rng();
    let mut i = start;
    let idx = rng.gen_range(start, end);
    
    swap(ilist, idx, end);
    
    for j in start..end {
        unsafe { comparisons += 1 }
        if ilist[j as usize] < ilist[end as usize] {
            swap(ilist, i, j);
            i += 1;
        }
    }
    
    swap(ilist, i, end);
    
    i
}

fn quicksort(ilist: &mut Vec<i64>, start: i64, end: i64) {
    if start < end {
        let idx = order(ilist, start, end);
    
        quicksort(ilist, start, idx - 1);
        quicksort(ilist, idx + 1, end);
    }
}

fn main() {
    let tries = 1000;
    let items = 10;
    let mut ilist: Vec<i64>;
    let mut rng = rand::thread_rng();
    let mut mean_time = 0.0;
    
    let mut acc = 0.0;
    for i in 1..items+1 {
        acc += 1.0 / (i as f64);
    }
    
    let expected = ((2 * items + 2) as f64) * acc - 4.0 * (items as f64);
    
    for _ in 0..tries {
        ilist = Vec::new();
        
        for _ in 0..items {
            ilist.push(rng.gen_range(0, items * 10));
        }
        
        let now = Instant::now();
        quicksort(&mut ilist, 0 as i64, (items - 1) as i64);
        
        mean_time += (now.elapsed().subsec_micros() as f64) / 100000.0
    }
        
    print!("Tiempo promedio por instancia: {} segundos.\n", 
          mean_time / (tries as f64));
        
    unsafe {
        print!("Comparaciones promedio: {}.\n", (comparisons as f64 / tries as f64))
    }
    
    print!("Valor esperado de comparaciones: {}.\n", expected)
}

