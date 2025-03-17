pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    if minefield.is_empty() {
        return result;
    }

    let height = minefield.len();
    let width = minefield[0].len();

    for row_idx in 0..height {
        let mut new_row = String::with_capacity(width);

        for col_idx in 0..width {
            let byte_char = minefield[row_idx].as_bytes()[col_idx];
            if byte_char == b'*' {
                new_row.push('*');
            } else {
                let mine_count = count_adjacent_mines(minefield, row_idx, col_idx, height, width);
                if mine_count > 0 {
                    new_row.push(char::from_digit(mine_count as u32, 10).unwrap());
                } else {
                    new_row.push(' ');
                }
            }
        }
        result.push(new_row);
    }

    result
}

fn count_adjacent_mines(
    minefield: &[&str],
    row_idx: usize,
    col_idx: usize,
    height: usize,
    width: usize,
) -> u8 {
    let mut count: u8 = 0;
    let row_start = row_idx.saturating_sub(1);
    let row_end = (row_idx + 1).min(height - 1);
    let col_start = col_idx.saturating_sub(1);
    let col_end = (col_idx + 1).min(width - 1);

    for r in row_start..=row_end {
        for c in col_start..=col_end {
            if r == row_idx && c == col_idx {
                continue;
            }

            if minefield[r].as_bytes()[c] == b'*' {
                count += 1;
            }
        }
    }
    count
}
