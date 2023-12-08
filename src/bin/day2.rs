use std::io::stdin;

fn main() -> std::io::Result<()> {
    let lines = stdin().lines();

    let mut sum = 0;
    let mut power_sum = 0;

    for line in lines {
        let line = line?;

        let (game, grabs) = line.split_once(':').unwrap();
        let game_n: usize = game.split_once(' ').unwrap().1.parse().unwrap();

        let grabs = grabs.split(';');
        let mut valid = true;

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for grab in grabs {
            let colors = grab.split(',');
            for count_color in colors {
                let (count, color) = count_color.trim().split_once(' ').unwrap();

                let count: usize = count.parse().unwrap();

                // Determine the max valid value for an allowed colour,
                // and re-assing the minimum value for that color as well.
                let max = match color {
                    "red" => {
                        min_red = min_red.max(count);
                        12
                    }
                    "green" => {
                        min_green = min_green.max(count);
                        13
                    }
                    "blue" => {
                        min_blue = min_blue.max(count);
                        14
                    }
                    _ => panic!(),
                };

                if count > max {
                    valid = false;
                }
            }
        }

        if valid {
            sum += game_n;
        }

        let power = min_red * min_green * min_blue;
        power_sum += power;
    }

    println!("Sum: {sum}");
    println!("Power sum: {power_sum}");

    Ok(())
}
