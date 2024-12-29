fn main() {
    let mut x = 5;
    { //Adding a scope to limit lifetime
        let y = &mut x; 
        *y += 1; 
    }
    { //Adding a scope to limit lifetime
        let z = &mut x; 
        *z += 1;
    }
    println!("x = {}", x);
}

//Alternative solution using shadowing
fn main() {
    let mut x = 5;
    let x = &mut x; 
    *x += 1; 
    let x = &mut x; 
    *x += 1;
    println!("x = {}", x);
} 