#[cfg(test)]
mod tests;

fn find_marker(input: &str, marker_len: usize) -> usize {
    let mut start_of_packet: usize = 0;
    loop {
        let slice = &input[start_of_packet..start_of_packet + marker_len];
        let num_of_duplicates = slice
            .char_indices()
            .filter(|(i, c)| slice.find(*c) != Some(*i))
            .count();
        if num_of_duplicates == 0 {
            break;
        }
        start_of_packet += 1;
    }

    start_of_packet + marker_len
}

fn part_one(input: &str) -> usize {
    find_marker(input, 4)
}

fn part_two(input: &str) -> usize {
    find_marker(input, 14)
}

pub fn combined(input: &str) -> (crate::Display, crate::Display) {
    (Box::new(part_one(input)), Box::new(part_two(input)))
}
