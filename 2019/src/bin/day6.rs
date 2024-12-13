use std::collections::HashMap;

fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    let mut sattelite_map = HashMap::new();

    for line in lines {
        let (body, sattelite) = line.split_once(')').unwrap();

        let body = sattelite_map.entry(body.to_string()).or_insert(Vec::new());
        body.push(sattelite.to_string());

        sattelite_map
            .entry(sattelite.to_string())
            .or_insert(Vec::new());
    }

    part1(&sattelite_map);
    part2(&sattelite_map);
}

fn part1(map: &HashMap<String, Vec<String>>) {
    let mut sum = 0;
    for planet in map.keys() {
        let mut count = 0;

        let mut parent = map.iter().find_map(|(p, s)| s.contains(planet).then(|| p));

        while let Some(planet_parent) = parent {
            count += 1;
            parent = map
                .iter()
                .find_map(|(p, s)| s.contains(planet_parent).then(|| p))
        }

        sum += count;
    }
    println!("Part 1: {sum}");
}

fn part2(map: &HashMap<String, Vec<String>>) {
    let you = "YOU".to_string();
    let santa = "SAN".to_string();

    let start = map
        .iter()
        .find_map(|(k, p)| p.contains(&you).then_some(k))
        .unwrap();

    let end = map
        .iter()
        .find_map(|(k, p)| p.contains(&santa).then_some(k))
        .unwrap();

    let path = &mut Vec::new();
    path_between(start, end, path, map);

    println!("{:?}", path.len());
}

fn path_between(
    start: &String,
    end: &String,
    path: &mut Vec<String>,
    map: &HashMap<String, Vec<String>>,
) -> bool {
    if start == end {
        return true;
    }

    path.push(start.to_string());
    if let Some(parent) = map.iter().find_map(|(k, p)| p.contains(start).then_some(k)) {
        if !path.contains(parent) {
            if path_between(parent, end, path, map) {
                return true;
            }
        }
    }

    for child in map.get(start).unwrap() {
        if path.contains(child) {
            continue;
        }

        if path_between(child, end, path, map) {
            return true;
        }
    }

    path.pop();
    return false;
}
