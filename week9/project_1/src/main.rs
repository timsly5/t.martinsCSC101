use std::fs::File;
use std::io::Write;

fn main() {
    // Define lager vector
    let mut lager: Vec<String> = Vec::new();

    lager.push("33 Export".to_string());
    lager.push("Desperados".to_string());
    lager.push("Goldberg".to_string());
    lager.push("Gulder".to_string());
    lager.push("Heineken".to_string());
    lager.push("Star".to_string());

    // Define stout vector
    let mut stout: Vec<String> = Vec::new();

    stout.push("Legend".to_string());
    stout.push("Turbo King".to_string());
    stout.push("Williams".to_string());

    // Define non-alcoholic vector
    let mut non_alcoholic: Vec<String> = Vec::new();

    non_alcoholic.push("Maltina".to_string());
    non_alcoholic.push("Amstel Malta".to_string());
    non_alcoholic.push("Malta Gold".to_string());
    non_alcoholic.push("Fayrouz".to_string());

    // Save vectors to respective files
    save_vec("lager.txt", &lager);
    save_vec("stout.txt", &stout);
    save_vec("non_alcoholic.txt", &non_alcoholic);
}

fn save_vec(filename: &str, vec: &Vec<String>) {
    let mut file = File::create(filename).unwrap();
    for item in vec {
        let line = format!("{}\n", item); // Add newline explicitly
        file.write_all(line.as_bytes()).unwrap();
    }
    println!("Saved {}", filename);
}


