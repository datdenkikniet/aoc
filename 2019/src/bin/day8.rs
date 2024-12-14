fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let pixels: Vec<_> = line.chars().map(|v| v as usize - '0' as usize).collect();

    part1(&pixels);
    part2(&pixels);
}

fn part1(pixels: &[usize]) {
    let width = 25;
    let height = 6;

    let layers: Vec<_> = pixels.chunks(width * height).collect();

    let min_layer = layers
        .iter()
        .min_by(|a, b| {
            let a_zeroes = a.iter().filter(|v| **v == 0).count();
            let b_zeroes = b.iter().filter(|v| **v == 0).count();

            a_zeroes.cmp(&b_zeroes)
        })
        .unwrap();

    let (ones, twos) = min_layer
        .iter()
        .fold((0usize, 0usize), |(ones, twos), item| {
            if *item == 1 {
                (ones + 1, twos)
            } else if *item == 2 {
                (ones, twos + 1)
            } else {
                (ones, twos)
            }
        });

    println!("Part 1: {}", ones * twos);
}

fn part2(pixels: &[usize]) {
    let width = 25;
    let height = 6;

    let layers: Vec<_> = pixels.chunks(width * height).collect();

    let mut output = vec![0; width * height];
    for idx in 0..(width * height) {
        for layer in layers.iter().rev() {
            if layer[idx] != 2 {
                output[idx] = layer[idx];
            }
        }
    }

    println!("Part 2:");
    for row in output.chunks(width) {
        for value in row {
            if *value == 0 {
                print!(" ");
            } else {
                print!("█");
            }
        }
        println!();
    }
    println!();
}
