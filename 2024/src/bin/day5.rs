use std::collections::HashMap;

fn main() {
    let input: Vec<String> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    let mut prerequisite_rules = HashMap::new();

    let mut input_iter = input.iter();
    for line in &mut input_iter {
        if line.is_empty() {
            break;
        }

        let (prerequisite, page) = line.split_once('|').unwrap();
        let (prerequisite, page): (usize, usize) =
            (prerequisite.parse().unwrap(), page.parse().unwrap());

        let prerequisites = prerequisite_rules.entry(page).or_insert(Vec::new());
        prerequisites.push(prerequisite);
    }

    let mut updates: Vec<Vec<usize>> = Vec::new();

    for full_update in input_iter {
        let update: Vec<usize> = full_update.split(',').map(|v| v.parse().unwrap()).collect();
        updates.push(update);
    }

    part1(&prerequisite_rules, &updates);
    part2(&prerequisite_rules, &updates);
}

fn is_valid(prerequisites: &HashMap<usize, Vec<usize>>, update: &[usize]) -> bool {
    let mut valid = true;
    for idx in 0..update.len() {
        valid &= missing_prerequisites(prerequisites, update, idx).is_empty();
    }
    valid
}

fn missing_prerequisites(
    prerequisites: &HashMap<usize, Vec<usize>>,
    update: &[usize],
    page_idx: usize,
) -> Vec<usize> {
    let page = update[page_idx];
    let previous_pages = &update[..page_idx];
    prerequisites
        .get(&page)
        .iter()
        .flat_map(|v| v.iter())
        .filter(|prerequisite| {
            !previous_pages.contains(prerequisite) && update.contains(prerequisite)
        })
        .map(|v| *v)
        .collect()
}

fn part1(prerequisites: &HashMap<usize, Vec<usize>>, updates: &Vec<Vec<usize>>) {
    let mut sum = 0;

    for update in updates {
        if is_valid(prerequisites, update) {
            assert!(update.len() % 2 == 1);
            let middle_page = &update[update.len() / 2];
            sum += *middle_page;
        }
    }

    println!("Part 1: {sum}");
}

fn part2(prerequisites: &HashMap<usize, Vec<usize>>, updates: &Vec<Vec<usize>>) {
    let mut sum = 0;

    for mut update in updates.iter().map(|v| v.clone()) {
        if is_valid(prerequisites, &update) {
            continue;
        }

        while !is_valid(prerequisites, &update) {
            for page_idx in 0..update.len() {
                let missing_prerequisites = missing_prerequisites(prerequisites, &update, page_idx);

                let mut insert_idx = None;
                for missing in missing_prerequisites {
                    for idx in page_idx..update.len() {
                        if update[idx] == missing {
                            insert_idx = Some(usize::max(*insert_idx.get_or_insert(idx), idx));
                            break;
                        }
                    }
                }

                if let Some(insert_idx) = insert_idx {
                    let page = update[page_idx];
                    update.remove(page_idx);
                    update.insert(insert_idx, page);
                }
            }
        }

        assert!(update.len() % 2 == 1);
        sum += update[update.len() / 2];
    }

    println!("Part 2: {sum}");
}
