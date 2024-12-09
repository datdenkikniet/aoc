use std::usize;

fn main() {
    let first_line = std::io::stdin().lines().map(|v| v.unwrap()).next().unwrap();
    let block_counts: Vec<usize> = first_line
        .chars()
        .map(|v| std::iter::once(v).collect::<String>().parse().unwrap())
        .collect();

    part1(&block_counts);
    part2(&block_counts);
}

fn part1(block_counts: &[usize]) {
    let mut block_id_map = parse(block_counts);

    let mut free_idx = block_id_map
        .iter()
        .enumerate()
        .find_map(|(idx, v)| v.is_none().then_some(idx))
        .unwrap();

    for idx in (0..block_id_map.len()).rev() {
        if idx <= free_idx {
            break;
        }

        if block_id_map[idx].is_some() {
            block_id_map.swap(idx, free_idx);

            free_idx = block_id_map
                .iter()
                .enumerate()
                .skip(free_idx)
                .find_map(|(idx, v)| v.is_none().then_some(idx))
                .unwrap()
        }
    }

    let checksum = checksum(&block_id_map);
    println!("Part 1: {checksum}");
}

fn part2(block_counts: &[usize]) {
    let mut blocks = Vec::new();
    let mut free_spaces = Vec::new();

    let mut offset = 0;
    let mut is_file_space = true;
    let mut file_id = 0;
    for len in block_counts {
        if is_file_space {
            blocks.push((file_id, offset, *len));
            file_id += 1;
        } else if *len != 0 {
            free_spaces.push((offset, *len));
        }

        offset += len;
        is_file_space = !is_file_space;
    }

    for (_, block_offset, block_len) in blocks.iter_mut().rev() {
        let orig = (*block_offset, *block_len);
        let mut swapped = false;
        for (fs_offset, fs_len) in free_spaces.iter_mut() {
            if *fs_offset >= *block_offset {
                continue;
            }

            if *fs_len < *block_len {
                continue;
            }

            *block_offset = *fs_offset;

            *fs_len -= *block_len;
            *fs_offset += *block_len;
            swapped = true;

            break;
        }

        if swapped {
            free_spaces.push(orig);
        }
    }

    let mut fs_compressed: Vec<(Option<usize>, usize, usize)> = free_spaces
        .into_iter()
        .map(|(offset, len)| (None, offset, len))
        .chain(
            blocks
                .into_iter()
                .map(|(block_id, offset, len)| (Some(block_id), offset, len)),
        )
        .collect();

    fs_compressed.sort_by(|(_, o1, _), (_, o2, _)| o1.cmp(o2));

    let fs: Vec<_> = fs_compressed
        .into_iter()
        .flat_map(|(id, _, len)| (0..len).map(move |_| id))
        .collect();

    let checksum = checksum(&fs);
    println!("Part 2: {:?}", checksum);
}

fn checksum(fs: &[Option<usize>]) -> usize {
    let mut sum = 0;
    for (idx, id) in fs.into_iter().enumerate() {
        if let Some(id) = id {
            sum += idx * id;
        }
    }
    sum
}

fn parse(block_counts: &[usize]) -> Vec<Option<usize>> {
    let mut is_file_space = true;
    let mut file_id = 0;
    let mut block_id_map: Vec<_> = Vec::new();
    for len in block_counts {
        if is_file_space {
            for _ in 0..*len {
                block_id_map.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..*len {
                block_id_map.push(None);
            }
        }

        is_file_space = !is_file_space;
    }

    block_id_map
}
