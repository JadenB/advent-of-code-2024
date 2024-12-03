use std::{
    cmp::Ordering,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    vec::Vec,
};

fn is_safe_pair(left: &i32, right: &i32, ordering: &Ordering) -> bool {
    left.cmp(right) == *ordering && (1..=3).contains(&(right - left).abs())
}

fn first_unsafe_pair_index<'a>(
    iter: impl Iterator<Item = &'a i32> + Clone,
    ordering: &Ordering,
) -> Option<usize> {
    iter.clone()
        .zip(iter.skip(1))
        .enumerate()
        .find_map(|(i, l)| (!is_safe_pair(l.0, l.1, ordering)).then_some(i))
}

struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let ordering = match self.levels.windows(2).next() {
            Some(w) => w[0].cmp(&w[1]),
            None => return true,
        };

        first_unsafe_pair_index(self.levels.iter(), &ordering).is_none()
    }

    fn is_safe_with_dampener(&self) -> bool {
        let removed_level_at_index_iter = |index| {
            self.levels
                .iter()
                .enumerate()
                .filter_map(move |(i, e)| (i != index).then_some(e))
        };

        // Since any level can be removed, we can't make any assumptions about the ordering.
        [Ordering::Less, Ordering::Greater].into_iter().any(
            |ordering| match first_unsafe_pair_index(self.levels.iter(), &ordering) {
                Some(index) => {
                    // If there is only one bad level and we encounter an unsafe pair, then
                    // removing one of the two will give us a safe report.
                    first_unsafe_pair_index(removed_level_at_index_iter(index), &ordering).is_none()
                        || first_unsafe_pair_index(
                            removed_level_at_index_iter(index + 1),
                            &ordering,
                        )
                        .is_none()
                }
                None => true,
            },
        )
    }
}

struct ProcessedInput {
    reports: Vec<Report>,
}

impl ProcessedInput {
    fn from_buf(reader: impl BufRead) -> Result<Self, Box<dyn Error>> {
        let reports = reader
            .lines()
            .map(|line| -> Result<Report, Box<dyn Error>> {
                let levels = line?
                    .split_whitespace()
                    .map(|x| x.parse())
                    .collect::<Result<_, _>>()?;
                Ok(Report { levels })
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { reports })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = ProcessedInput::from_buf(BufReader::new(File::open("input/input.txt")?))?;

    // Part 1
    let safe_report_count = input.reports.iter().filter(|r| r.is_safe()).count();
    println!("{safe_report_count}");

    // Part 2
    // Runs in O(N) since we iterate through each report a maximum of six times.
    let safe_report_count = input
        .reports
        .iter()
        .filter(|r| r.is_safe_with_dampener())
        .count();
    println!("{safe_report_count}");

    Ok(())
}
