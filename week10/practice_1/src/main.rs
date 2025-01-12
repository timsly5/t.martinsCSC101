fn main () {
    let v = vec![101, 250, 330, 400];
    // vector v is the owner of the object in heap/ above

    // only a single variable owns the heap memory at any given time
    let v2 = v;
    
    println!("{:?}", v2);

} 
