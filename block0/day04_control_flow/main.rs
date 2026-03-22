// AnimaLab — day04_control_flow
fn main() {
    let score = 85;

    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        _ => "F",
    };

    println!("Score: {}, Grade: {}", score, grade);
}
