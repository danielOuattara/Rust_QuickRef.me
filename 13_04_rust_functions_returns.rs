/*

fn calculate(radius: f64, pi: f64) -> (f64, f64) {
    let perimeter = 2.0 * pi * radius;
    let area = pi * radius * radius;
    return (area, perimeter);
}

fn main() {
    let (radius, pi) = (3.0, 3.14);
    let (area, _perimeter) = calculate(radius, pi);
    println!("The area and the perimeter of the circle are: {area} & {_perimeter}");
}

 */

// OR

fn calculate(radius: &mut f64, pi: &mut f64) -> (f64, f64) {
    let perimeter = 2.0 * *pi * *radius;
    let area = *pi * *radius * *radius;
    return (area, perimeter);
}

fn main() {
    let (mut radius, mut pi) = (3.0, 3.14);
    let (area, _perimeter) = calculate(&mut radius, &mut pi);
    println!("The area and the perimeter of the circle are: {area} & {_perimeter}");
}
