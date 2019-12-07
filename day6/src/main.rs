use std::collections::{HashMap, VecDeque};

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> DynResult<()> {
    let file = std::fs::read("day6-input.txt")?;
    let data = file
        .split(|c| *c as char == '\n')
        .filter(|line| line.len() > 0);

    let mut orbiters_map = HashMap::<u64, Vec<u64>>::new();
    let mut orbits_map = HashMap::<u64, u64>::new();
    let mut orbits_count_map = HashMap::<u64, u64>::new();

    for orbit_data in data {
        let mut split = orbit_data.split(|c| *c as char == ')');
        let lhs = split.next().unwrap_or_else(|| {
            panic!(
                "Missing orbit center in '{}'",
                std::str::from_utf8(orbit_data).unwrap()
            )
        });
        let rhs = split.next().unwrap_or_else(|| {
            panic!(
                "Missing orbiting body in '{}'",
                std::str::from_utf8(orbit_data).unwrap()
            )
        });
        assert_eq!(split.next(), None);
        let lhs = as_num(lhs);
        let rhs = as_num(rhs);
        orbiters_map
            .entry(lhs)
            .and_modify(|vec| vec.push(rhs))
            .or_insert_with(|| vec![rhs]);
        orbits_map.insert(rhs, lhs);
    }

    let mut queue = VecDeque::new();

    for body in orbiters_map.keys() {
        // start processing from root orbited entries
        if !orbits_map.contains_key(&body) {
            queue.push_back(body);
        }
    }

    while let Some(body) = queue.pop_front() {
        if let Some(orbited_body) = orbits_map.get(body) {
            orbits_count_map.insert(*body, 1 + *orbits_count_map.get(orbited_body).unwrap());
        } else {
            orbits_count_map.insert(*body, 0);
        }
        if let Some(orbiters) = orbiters_map.get(body) {
            queue.extend(orbiters);
        }
    }

    let total_orbit_relations: u64 = orbits_count_map.values().sum();
    println!("total orbit relations: {}", total_orbit_relations);

    let you = as_num("YOU".as_bytes());
    let san = as_num("SAN".as_bytes());

    let mut parent_dist_map = HashMap::new();
    let mut current = you;
    let mut path_len = 0;
    parent_dist_map.insert(you, 0);
    while let Some(parent) = orbits_map.get(&current) {
        path_len += 1;
        current = *parent;
        parent_dist_map.insert(*parent, path_len);
    }

    let mut current = san;
    let mut path_len = 0;
    while let Some(parent) = orbits_map.get(&current) {
        path_len += 1;
        if let Some(path_to_you) = parent_dist_map.get(parent) {
            println!("Distance: {}", path_to_you + path_len - 2);
            break;
        }
        current = *parent;
    }

    Ok(())
}

fn as_num(slice: &[u8]) -> u64 {
    assert!(slice.len() <= 8);
    let mut num = 0;
    for i in 0..8 {
        num |= (*slice.get(i).unwrap_or(&0) as u64) << (i * 8);
    }
    num
}
