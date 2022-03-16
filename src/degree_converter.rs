use std::io;

pub fn degree_converter() -> () {
    println!("Would you like to convert Fahrenheit or Celius?");
    println!("Type F for Fahrenheit or C for Celius: ");

    let mut degree_type = String::new();

    io::stdin()
        .read_line(&mut degree_type)
        .expect("Failed to read");

    println!("Please input the current degree");

    let mut degree = String::new();

    io::stdin().read_line(&mut degree).expect("Failed to read");

    let degree: f32 = degree.trim().parse().expect("Please type a number!");
    let new_degree: f32 = calculate_degree(degree_type, degree);

    println!("The new degree is {}", new_degree);
}

fn calculate_degree(degree_type: String, degree: f32) -> f32 {
    let fstring: String = String::from("F");
    if degree_type.trim().eq(&fstring) {
        return (degree - 32.0) * 0.5556;
    } else {
        return (degree / 0.5556) + 32.0;
    }
}
