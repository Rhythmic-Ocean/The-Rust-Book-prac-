use std::io;
use std::collections::HashMap;

fn main() {

    println!("How many integers would u like to enter?");
    let mut num = String::new();
    io::stdin().read_line(&mut num)
        .expect("Failed to read input");

    let num: usize = num.trim().parse()
        .expect("Please try a number");
    let mut v = Vec::new();
    for i in 0..num{
        println!("Enter {}st number", i);
        let mut dig = String::new();
        io::stdin().read_line(&mut dig)
            .expect( "Failed to read input!");
        let digg: i32 = dig.trim().parse()
            .expect("Print enter a number");
        v.insert(i, digg);
    }
    let avg = avg(&v);
    println!("The avg is: {}", avg);

    {
        let median = median(&mut v);
        println!("The median is: {}", median);
    }

    let m = mode(&v);
    println!("The mode is: {}", m);


}

fn avg(v: &Vec<i32>) -> f64{
    let mut sum:f64 = 0.0;
    let mut count:f64 = 0.0;
    for i in v{ //cuz all value of vector is stored in heap, if I did not give it &, the original vector will be invalid
        let m:f64 = *i as f64;
        sum += m;
        count += 1.0;
    }
    let avg: f64 = sum/count;
    avg
}

fn median(v: &mut Vec<i32>) -> i32{
    let mut count:f32 = 0.0;
    for i in 0..v.len(){
        for j in 0..v.len(){
            if v[i] > v[j]{
                let k = v[i];
                v[i] = v[j];
                v[j] = k;
            }
        }
        count += 1.0;
    }
    let median_index = (count/2.0).round() as usize;
    *(&v[median_index])
}

fn mode(v: &Vec<i32>) -> i32{
    let mut h = HashMap::new();
    for i in v{
        let p = h.entry(*i).or_insert(0);
        *p += 1;
    }
    let mut hp = (0,0);
    for (item, val) in &h{
        if *val > hp.1{
            hp.0 = *item;
            hp.1 = *val;
        }
    }
    hp.0
}