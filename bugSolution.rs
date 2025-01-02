fn main() {
    let mut x = 5;

    // Option 1: Use mutable reference only
    { 
        let y = &mut x;
        *y += 1;
        println!("x (Option 1) = {}", x); 
    }

    // Option 2: Use immutable reference only
    {  
        let z = &x;
        println!("x (Option 2) = {}", *z); 
    }

    //Option 3: Separate scopes to avoid simultaneous borrows
    { 
        let y = &mut x;
        *y += 1;
    }
    {  
        let z = &x;
        println!("x (Option 3) = {}", *z); 
    }
} 