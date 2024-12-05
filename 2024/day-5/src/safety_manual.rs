use std::collections::{HashMap, HashSet, VecDeque};

pub struct ProductionInstructions {
    rules: Vec<Rule>,
    pages: Vec<Page>,
}

impl ProductionInstructions {
    pub fn new(rules: Vec<Rule>, pages: Vec<Page>) -> Self {
        Self { rules, pages }
    }

    pub fn sum_of_middle_page_numbers(&self) -> u32 {
        self.pages
            .iter()
            .filter(|page| page.is_correctly_ordered(&self.rules))
            .map(|page| page.middle_page_number())
            .sum()
    }
}

pub struct Rule(u32, u32);

impl Rule {
    pub fn new(from: u32, to: u32) -> Self {
        Self(from, to)
    }
}

impl From<(u32, u32)> for Rule {
    fn from(tuple: (u32, u32)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

pub struct Page {
    numbers: Vec<u32>,
}

impl Page {
    pub fn new(numbers: Vec<u32>) -> Self {
        Self { numbers }
    }

    pub fn middle_page_number(&self) -> u32 {
        self.numbers[self.numbers.len() / 2]
    }

    pub fn is_correctly_ordered(&self, rules: &[Rule]) -> bool {
        let position: HashMap<u32, usize> = self
            .numbers
            .iter()
            .enumerate()
            .map(|(i, &number)| (number, i))
            .collect();
        for &Rule(x, y) in rules {
            if let (Some(&pos_x), Some(&pos_y)) = (position.get(&x), position.get(&y)) {
                if pos_x > pos_y {
                    return false;
                }
            }
        }
        true
    }

    pub fn correct_order(&self, rules: &[Rule]) -> Page {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut in_degree: HashMap<u32, usize> = HashMap::new();
        let pages: HashSet<u32> = self.numbers.iter().cloned().collect();

        for &page in &pages {
            graph.entry(page).or_default();
            in_degree.entry(page).or_insert(0);
        }

        for &Rule(from, to) in rules {
            if pages.contains(&from) && pages.contains(&to) {
                graph.entry(from).or_default().push(to);
                *in_degree.entry(to).or_insert(0) += 1;
            }
        }

        let mut queue: VecDeque<u32> = in_degree
            .iter()
            .filter(|&(_, &deg)| deg == 0)
            .map(|(&page, _)| page)
            .collect();

        let mut sorted = Vec::new();
        while let Some(page) = queue.pop_front() {
            sorted.push(page);
            if let Some(neighbors) = graph.get(&page) {
                for &neighbor in neighbors {
                    let degree = in_degree.get_mut(&neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        Page::new(sorted)
    }
}

impl From<Vec<u32>> for Page {
    fn from(numbers: Vec<u32>) -> Self {
        Self { numbers }
    }
}
