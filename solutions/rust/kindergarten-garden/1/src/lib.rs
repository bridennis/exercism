pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let codes = ['G', 'C', 'R', 'V'];
    let plants = ["grass", "clover", "radishes", "violets"];

    let mut resp: Vec<&str> = vec![];
    if let Some(student_pos) = students.iter().position(|&x| x == student) {
        for s in diagram.split("\n") {
            for offset in 0..=1 {
                let code = s.chars().nth(student_pos * 2 + offset).unwrap();
                if let Some(plant_pos) = codes.iter().position(|&x| x == code) {
                    resp.push(plants[plant_pos]);
                }
            }
        }
    }

    resp
}
