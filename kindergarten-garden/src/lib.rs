pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_index = match student {
        "Alice" => 0,
        "Bob" => 1,
        "Charlie" => 2,
        "David" => 3,
        "Eve" => 4,
        "Fred" => 5,
        "Ginny" => 6,
        "Harriet" => 7,
        "Ileana" => 8,
        "Joseph" => 9,
        "Kincaid" => 10,
        "Larry" => 11,
        _ => panic!("Unknown student"),
    };

    let plant_start_index = student_index * 2;

    let mut lines = diagram.split('\n');
    let first_row = lines.next().unwrap();
    let second_row = lines.next().unwrap();

    let plant_codes = [
        first_row.chars().nth(plant_start_index).unwrap(),
        first_row.chars().nth(plant_start_index + 1).unwrap(),
        second_row.chars().nth(plant_start_index).unwrap(),
        second_row.chars().nth(plant_start_index + 1).unwrap(),
    ];

    plant_codes
        .iter()
        .map(|&code| match code {
            'V' => "violets",
            'R' => "radishes",
            'C' => "clover",
            'G' => "grass",
            _ => panic!("Unknown plant code"),
        })
        .collect()
}
