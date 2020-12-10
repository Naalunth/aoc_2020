use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, opt, recognize, value},
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use petgraph::{
    graph::{DiGraph, NodeIndex},
    visit::{Dfs, Walker},
};
use std::collections::{HashMap, HashSet};

#[aoc(day7, part1)]
pub fn part_1(input: &str) -> usize {
    let (graph, node_id) = create_graph(input);

    Dfs::new(&graph, node_id["shiny gold"])
        .iter(&graph)
        .collect::<HashSet<_>>()
        .into_iter()
        .count()
        - 1
}

#[aoc(day7, part2)]
pub fn part_2(input: &str) -> usize {
    let map = parse_input(input);
    let shiny_gold_name = "shiny gold".to_owned();
    let mut stack = vec![(&shiny_gold_name, 1usize)];
    let mut total = 0usize;

    while let Some((bag_name, count)) = stack.pop() {
        total += count;
        stack.extend(map[bag_name].iter().map(|(b, c)| (b, *c * count)));
    }

    total - 1
}

fn create_graph(input: &str) -> (DiGraph<String, usize, u32>, HashMap<String, NodeIndex<u32>>) {
    let map = parse_input(input);

    let mut graph = DiGraph::<String, usize, _>::new();

    let node_id = map
        .iter()
        .flat_map(|(container, children)| std::iter::once(container).chain(children.keys()))
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|bag_name| (bag_name.clone(), graph.add_node(bag_name.clone())))
        .collect::<HashMap<_, _>>();

    graph.extend_with_edges(map.into_iter().flat_map(|(container, children)| {
        children.into_iter().map({
            let node_id = &node_id;
            move |(child, count)| (node_id[&child], node_id[&container], count)
        })
    }));
    (graph, node_id)
}

fn parse_input(input: &str) -> HashMap<String, HashMap<String, usize>> {
    input
        .split('\n')
        .map(|line| parse_line(line).unwrap().1)
        .collect::<HashMap<_, _>>()
}

fn parse_line(i: &str) -> IResult<&str, (String, HashMap<String, usize>)> {
    let mut bag_name_parser = recognize(tuple((alpha1, tag(" "), alpha1)));
    let (i, bag_name) = bag_name_parser(i)?;
    let (i, _) = tag(" bags contain ")(i)?;
    let (i, children) = alt((
        value(HashMap::new(), tag("no other bags")),
        map(
            separated_list1(
                tag(", "),
                terminated(
                    map(
                        separated_pair(
                            map(digit1, |s: &str| s.parse::<usize>().unwrap()),
                            tag(" "),
                            bag_name_parser,
                        ),
                        |(c, n)| (n.to_owned(), c),
                    ),
                    tuple((tag(" bag"), opt(tag("s")))),
                ),
            ),
            |list| list.into_iter().collect(),
        ),
    ))(i)?;
    let (i, _) = tag(".")(i)?;
    Ok((i, (bag_name.to_owned(), children)))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &'static str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(EXAMPLE), 4);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(EXAMPLE), 32);
        assert_eq!(part_2("shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."), 126);
    }
}
