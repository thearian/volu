pub fn count_to_letter(count: u8, letter: char) -> String {
    (0..count)
        .map(|_| letter)
        .collect::<String>()
}

pub fn produce_letter(space_count: u8, occupied: u8, letter: char) -> String {
    count_to_letter(
        if space_count > occupied {space_count - occupied}
        else {0},
        letter
    )
}