use std::collections::VecDeque;

fn main() {
    let test_data = include_str!("test.txt");
    let actual_data = include_str!("input.txt");

    let t1 = part_1(test_data);
    println!("[TEST 1]: {t1}");

    let p1 = part_1(actual_data);
    println!("[PART 1]: {p1}");

    let t2 = part_2(test_data);
    println!("[TEST 2]: {t2}");

    let p2 = part_2(actual_data);
    println!("[PART 2]: {p2}");
}

fn part_1(input: &str) -> usize {
    parse_input(input).into_iter().map(get_hash).sum()
}

fn part_2(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .fold(BoxMap::new(), BoxMap::parse_op)
        .calculate_focus()
}

fn get_hash(s: &str) -> usize {
    let hash = |curr: usize, c: char| ((curr + (c as usize)) * 17) % 256;
    s.chars().fold(0, hash)
}

fn parse_input(input: &str) -> Vec<&str> {
    input.split(",").map(str::trim).collect()
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_size: usize,
}

impl Lens {
    fn new(label: &str, focal_size: usize) -> Self {
        Self {
            label: label.to_string(),
            focal_size,
        }
    }
}

#[derive(Clone, Debug)]
struct LensList {
    list: VecDeque<Lens>,
}

impl LensList {
    fn new() -> Self {
        Self {
            list: VecDeque::new(),
        }
    }

    fn get_focus(&self) -> usize {
        self.list
            .iter()
            .zip(1..)
            .map(|(l, i)| i * l.focal_size)
            .sum()
    }

    fn add(&mut self, new_lens: Lens) {
        for lens in self.list.iter_mut() {
            if lens.label == new_lens.label {
                lens.focal_size = new_lens.focal_size;
                return;
            }
        }
        self.list.push_back(new_lens)
    }

    fn remove(&mut self, label: &str) {
        for i in 0..self.list.len() {
            if self.list[i].label == label {
                self.list.remove(i);
                return;
            }
        }
    }
}

#[derive(Clone, Debug)]
struct BoxMap {
    boxes: Vec<LensList>,
}

impl BoxMap {
    fn new() -> Self {
        Self {
            boxes: vec![LensList::new(); 256],
        }
    }

    fn calculate_focus(&self) -> usize {
        self.boxes
            .iter()
            .zip(1..)
            .map(|(b, i)| i * b.get_focus())
            .sum()
    }

    fn parse_op(mut self, op: &str) -> Self {
        let idx = op
            .char_indices()
            .find_map(|(i, c)| ['-', '='].contains(&c).then(|| i))
            .unwrap();

        let (label, op) = op.split_at(idx);
        let label_hash = Self::get_hash(label);

        let lens_box = &mut self.boxes[label_hash];

        match op {
            "-" => lens_box.remove(label),
            _ => {
                let (_, d) = op.split_at(1);
                let focal_size = d.parse().unwrap();
                let new_lens = Lens::new(label, focal_size);
                lens_box.add(new_lens)
            }
        }

        self
    }

    fn get_hash(s: &str) -> usize {
        let hash = |curr: usize, c: char| ((curr + (c as usize)) * 17) % 256;
        s.chars().fold(0, hash)
    }
}
