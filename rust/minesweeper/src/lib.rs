static NEIGBOURHOOD_OFFSETS: &[(i32, i32)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let f = |x: i32, y: i32| -> Option<char> {
        if x < 0 || y < 0 || x >= minefield.len() as i32 || y >= minefield[x as usize].len() as i32
        {
            return None;
        }
        minefield[x as usize].chars().nth(y as usize)
    };

    (0..minefield.len())
        .map(|x| {
            (0..minefield[x].len())
                .map(|y| {
                    let (x, y) = (x as i32, y as i32);
                    if let Some(c) = f(x, y) {
                        if c == '*' {
                            return '*';
                        }
                    }
                    match NEIGBOURHOOD_OFFSETS
                        .iter()
                        .map(|(dx, dy)| {
                            f(x + dx, y + dy)
                                .map(|c| if c == '*' { 1 } else { 0 })
                                .unwrap_or(0)
                        })
                        .sum::<u32>()
                    {
                        0 => ' ',
                        c => char::from_digit(c, 10).unwrap(),
                    }
                })
                .collect()
        })
        .collect()
}
