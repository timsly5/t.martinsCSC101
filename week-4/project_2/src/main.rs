fn calculate_incentive(experienced: bool, age: u32) -> u32 {
    if experienced {
        if age >= 40 {
            1_560_000 // Incentive for experienced employees aged 40 or more
        } else if age >= 30 && age < 40 {
            1_480_000 // Incentive for experienced employees aged 30-39
        } else if age < 28 {
            1_300_000 * 12 // Monthly incentive of N1,300,000 multiplied by 12 for annual
        } else {
            0 // No specific incentive for experienced employees aged 28-29
        }
    } else {
        100_000 // Incentive for inexperienced employees
    }
}

fn main() {
    let experienced = true; // Set to true if the employee is experienced, false otherwise
    let age = 35; // Set the age of the employee

    let incentive = calculate_incentive(experienced, age);
    println!("The annual incentive for the employee is: N{}", incentive);
}


