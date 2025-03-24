pub fn build_proverb(list: &[&str]) -> String {
    if !list.is_empty() {
        let main_lines: Vec<String> = list
            .windows(2)
            .map(|pair| format!("For want of a {} the {} was lost.", pair[0], pair[1]))
            .collect();

        let last_line = format!("And all for the want of a {}.", list[0]);

        if !main_lines.is_empty() {
            format!("{}\n{}", main_lines.join("\n"), last_line)
        } else {
            last_line
        }
    } else {
        String::new()
    }
}
