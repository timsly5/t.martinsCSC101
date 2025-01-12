fn main(){

    let x = vec![500,200,300];
    borrow_vector(&x); //passing by reference

    println!("Printing the value from main() x[0]={}",x[0]);
    println!("****************************");
}

fn borrow_vector(z:&Vec<i32>){

    println!("*****************************");
    println!("Inside borrow_vector function {:?}",z);
    println!("------------------------------");
}
