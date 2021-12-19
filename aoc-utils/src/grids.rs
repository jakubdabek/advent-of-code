use std::fmt::Debug;

pub fn position_in_grid<T>(
    grid: &[impl AsRef<[T]>],
    mut pred: impl FnMut(&T) -> bool,
) -> Option<(usize, usize)> {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| row.as_ref().iter().position(&mut pred).map(|x| (x, y)))
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn print_digit(x: &u8) {
    match x {
        0..=9 => print!("{}", x),
        10 => print!("*"),
        11 => print!("@"),
        12 => print!("#"),
        254 => print!("-"),
        255 => print!("+"),
        _ => print!("%"),
    }
}

pub fn print_grid<T: Debug>(grid: &[impl AsRef<[T]>], print: impl Fn(&T)) {
    for row in grid {
        for v in row.as_ref() {
            // print!("{:?}", v);
            print(v);
        }
        println!();
    }
}

pub fn for_neighbours_4(
    (x, y): (usize, usize),
    (width, height): (usize, usize),
    mut f: impl FnMut((usize, usize)),
) {
    if x + 1 < width {
        f((x + 1, y));
    }
    if y + 1 < height {
        f((x, y + 1));
    }
    if x > 0 {
        f((x - 1, y));
    }
    if y > 0 {
        f((x, y - 1));
    }
}

pub fn for_neighbours_8(
    (x, y): (usize, usize),
    (width, height): (usize, usize),
    mut f: impl FnMut((usize, usize)),
) {
    let y_down = Some(y).filter(|&y| y > 0).map(|y| y - 1);
    let y_up = Some(y).filter(|&y| y + 1 < height).map(|y| y + 1);
    for current_y in [y_down, Some(y), y_up].into_iter().flatten() {
        if x > 0 {
            f((x - 1, current_y));
        }
        if current_y != y {
            f((x, current_y));
        }
        if x + 1 < width {
            f((x + 1, current_y));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn check_4(
        xy: (usize, usize),
        hw: (usize, usize),
        expected: impl IntoIterator<Item = (usize, usize)>,
    ) {
        let mut neighbours = HashSet::new();
        for_neighbours_4(xy, hw, |pos| drop(neighbours.insert(pos)));
        assert_eq!(neighbours, expected.into_iter().collect())
    }

    #[test]
    fn neighbours_4() {
        for x in 1..9 {
            for y in 1..9 {
                check_4(
                    (x, y),
                    (10, 10),
                    [(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)],
                );
                check_4((0, y), (10, 10), [(0, y - 1), (0, y + 1), (1, y)]);
                check_4((9, y), (10, 10), [(9, y - 1), (9, y + 1), (8, y)]);
            }
            check_4((x, 0), (10, 10), [(x + 1, 0), (x - 1, 0), (x, 1)]);
            check_4((x, 9), (10, 10), [(x + 1, 9), (x - 1, 9), (x, 8)]);
        }
    }

    fn check_8(
        xy: (usize, usize),
        hw: (usize, usize),
        expected: impl IntoIterator<Item = (usize, usize)>,
    ) {
        let mut neighbours = HashSet::new();
        for_neighbours_8(xy, hw, |pos| drop(neighbours.insert(pos)));
        assert_eq!(neighbours, expected.into_iter().collect())
    }

    #[rustfmt::skip]
    #[test]
    fn neighbours_8() {
        for x in 1..9 {
            for y in 1..9 {
                check_8(
                    (x, y),
                    (10, 10),
                    [
                        (x + 1, y), (x - 1, y),
                        (x + 1, y + 1), (x - 1, y + 1),
                        (x + 1, y - 1), (x - 1, y - 1),
                        (x, y - 1), (x, y + 1),
                    ],
                );
                // check_8((0, y), (10, 10), [(0, y - 1), (0, y + 1), (1, y)]);
                // check_8((9, y), (10, 10), [(9, y - 1), (9, y + 1), (8, y)]);
            }
            // check_8((x, 0), (10, 10), [(x + 1, 0), (x - 1, 0), (x, 1)]);
            // check_8((x, 9), (10, 10), [(x + 1, 9), (x - 1, 9), (x, 8)]);
        }
    }
}
