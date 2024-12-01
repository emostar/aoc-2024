advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let mut list1 = vec![];
    let mut list2 = vec![];

    let re = regex::Regex::new(r"(\d+)\s+(\d+)").unwrap();

    re.captures_iter(input).for_each(|cap| {
        list1.push(cap[1].parse::<u32>().unwrap());
        list2.push(cap[2].parse::<u32>().unwrap());
    });

    assert_eq!(list1.len(), list2.len());

    // sort both lists
    list1.sort();
    list2.sort();

    // Go through each list doing the calculation to get the distance and sum the result

    for i in 0..list1.len() {
        // Get absolute distance
        let distance = list1[i].abs_diff(list2[i]);
        result += distance;
    }

    Some(result)
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let mut list1 = vec![];
    let mut list2_count_by_value = std::collections::HashMap::new();

    let re = regex::Regex::new(r"(\d+)\s+(\d+)").unwrap();

    re.captures_iter(input).for_each(|cap| {
        list1.push(cap[1].parse::<u32>().unwrap());
        let val = cap[2].parse::<u32>().unwrap();
        *list2_count_by_value.entry(val).or_insert(0) += 1;
    });

    for i in 0..list1.len() {
        let val = list1[i];
        let count = list2_count_by_value.get(&val).unwrap_or(&0);
        result += count * val;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
