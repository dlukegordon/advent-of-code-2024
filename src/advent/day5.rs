use crate::get_data_filepath;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

type Page = u32;
type Rule = (Page, Page);
type Rules = Vec<Rule>;
type Graph = HashMap<Page, Vec<Page>>;
type Update = Vec<Page>;
type Data = (Vec<Rule>, Vec<Update>);

pub fn run() {
    let data_filepath = get_data_filepath!();
    let data = get_data(&data_filepath);

    let answer1 = part1(&data);
    println!("Part 1: {answer1}");

    let answer2 = part2(&data);
    println!("Part 2: {answer2}");
}

fn get_data(data_filepath: &Path) -> Data {
    let data_str = fs::read_to_string(data_filepath).unwrap();
    get_data_from_str(data_str)
}

fn get_data_from_str(data_str: String) -> Data {
    let parts: Vec<&str> = data_str.split("\n\n").collect();

    let rules = parts[0]
        .lines()
        .map(|line| {
            let nums: Vec<Page> = line.split('|').map(|x| x.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    let updates = parts[1]
        .lines()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn build_subgraph(rules: &Rules, update: &Update) -> Graph {
    let pages: HashSet<Page> = update.iter().cloned().collect();
    let mut graph: Graph = HashMap::new();

    for &(a, b) in rules {
        if pages.contains(&a) && pages.contains(&b) {
            graph.entry(b).or_default().push(a);
        }
    }

    graph
}

fn update_is_correctly_ordered(graph: &Graph, update: &Update) -> bool {
    let mut visited: HashSet<Page> = HashSet::new();

    for &current_page in update.iter() {
        let predecessors = match graph.get(&current_page) {
            Some(predecessors) => predecessors,
            None => &vec![],
        };

        for &predecessor in predecessors {
            if !visited.contains(&predecessor) {
                return false;
            }
        }

        visited.insert(current_page);
    }

    true
}

fn find_new_leaf_node(graph: &Graph, update: &Update, visited: &Update) -> Page {
    for page in update {
        if !graph.contains_key(page) && !visited.contains(page) {
            return *page;
        }
    }

    panic!("Could not find leaf node");
}

fn prune_graph(graph: &Graph, to_prune: Page) -> Graph {
    let mut pruned_graph = HashMap::new();

    for (&k, predecessors) in graph.iter() {
        let pruned_vec: Vec<Page> = predecessors
            .iter()
            .cloned()
            .filter(|&page| page != to_prune)
            .collect();
        if !pruned_vec.is_empty() {
            pruned_graph.insert(k, pruned_vec);
        }
    }

    pruned_graph
}

fn sort_update(graph: &Graph, update: &Update) -> Update {
    let mut graph = graph.clone();
    let mut sorted = vec![];

    while sorted.len() < update.len() {
        let leaf = find_new_leaf_node(&graph, update, &sorted);
        sorted.push(leaf);
        graph = prune_graph(&graph, leaf);
    }

    sorted
}

fn part1((rules, updates): &Data) -> u64 {
    let mut sum_of_middle_pages = 0;

    for update in updates {
        let subgraph = build_subgraph(rules, update);

        if update_is_correctly_ordered(&subgraph, update) {
            let middle_page = update[update.len() / 2];
            sum_of_middle_pages += middle_page as u64;
        }
    }

    sum_of_middle_pages
}

fn part2((rules, updates): &Data) -> u64 {
    let mut sum_of_middle_pages = 0;

    for update in updates {
        let subgraph = build_subgraph(rules, update);

        if !update_is_correctly_ordered(&subgraph, update) {
            let sorted_update = sort_update(&subgraph, update);
            let middle_page = sorted_update[sorted_update.len() / 2];
            sum_of_middle_pages += middle_page as u64;
        }
    }

    sum_of_middle_pages
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_STR: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        let data = get_data_from_str(TEST_DATA_STR.into());
        let answer = part1(&data);
        assert_eq!(answer, 143)
    }

    #[test]
    fn test_part2() {
        let data = get_data_from_str(TEST_DATA_STR.into());
        let answer = part2(&data);
        assert_eq!(answer, 123)
    }
}
