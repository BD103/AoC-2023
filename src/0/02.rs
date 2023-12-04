static INPUT: &str = include_str!("../../inputs/0/02.in");

fn main() {
    // part_a();
    part_b();
}

fn part_a() {
    let mut sum = 0;

    for line in INPUT.lines() {
        let (game, sets) = line.split_once(": ").unwrap();

        // Get the game number
        let game = u64::from_str_radix(game.split_once(' ').unwrap().1, 10).unwrap();

        let mut max_rgb = (0, 0, 0);

        // Calculate largest amount of each color pulled
        for subset in sets.split("; ") {
            for cubes in subset.split(", ") {
                let (amt, kind) = cubes.split_once(' ').unwrap();
                let amt = u64::from_str_radix(amt, 10).unwrap();

                match kind {
                    "red" => max_rgb.0 = max_rgb.0.max(amt),
                    "green" => max_rgb.1 = max_rgb.1.max(amt),
                    "blue" => max_rgb.2 = max_rgb.2.max(amt),
                    _ => unreachable!(),
                };
            }
        }

        if max_rgb.0 <= 12 && max_rgb.1 <= 13 && max_rgb.2 <= 14 {
            println!("Possible game: {}", game);
            sum += game;
        }
    }

    println!("Sum: {}", sum);
}

fn part_b() {
    let mut sum = 0;

    for line in INPUT.lines() {
        let (game, sets) = line.split_once(": ").unwrap();

        // Get the game number
        let game = u64::from_str_radix(game.split_once(' ').unwrap().1, 10).unwrap();

        let mut max_rgb = (0, 0, 0);

        // Calculate largest amount of each color pulled
        for subset in sets.split("; ") {
            for cubes in subset.split(", ") {
                let (amt, kind) = cubes.split_once(' ').unwrap();
                let amt = u64::from_str_radix(amt, 10).unwrap();

                match kind {
                    "red" => max_rgb.0 = max_rgb.0.max(amt),
                    "green" => max_rgb.1 = max_rgb.1.max(amt),
                    "blue" => max_rgb.2 = max_rgb.2.max(amt),
                    _ => unreachable!(),
                };
            }
        }

        let power = max_rgb.0 * max_rgb.1 * max_rgb.2;
        sum += power;
    }

    println!("Sum: {}", sum);
}
