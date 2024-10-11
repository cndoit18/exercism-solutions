pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let index = 2*students.iter().position(|&s| s == student).unwrap();
    diagram
        .lines()
        .flat_map(|row| {
            row
                .get(index..=index + 1)
                .unwrap()
                .chars()
                .map(|c| match c {
                    'G' => "grass",
                    'C' => "clover",
                    'R' => "radishes",
                    'V' => "violets",
                    _ => panic!("Invalid plant"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
