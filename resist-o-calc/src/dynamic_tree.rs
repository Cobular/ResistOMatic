use std::io::Cursor;

static EPS: f32 = 0.01;
static MAX_MUL: f32 = 10_000.0;

struct ResistorPair<'a> {
    data: &'a [f32],
    top_idx: usize,
    bottom_idx: usize,
    top_mul: f32,
    bottom_mul: f32,
}

impl<'a> ResistorPair<'a> {
    fn new(
        data: &'a [f32],
        top_idx: usize,
        bottom_idx: usize,
        top_mul: f32,
        bottom_mul: f32,
    ) -> Self {
        Self {
            data,
            top_idx,
            bottom_idx,
            top_mul,
            bottom_mul,
        }
    }

    fn top_value(&self) -> f32 {
        self.data[self.top_idx] * self.top_mul
    }

    fn bottom_value(&self) -> f32 {
        self.data[self.bottom_idx] * self.bottom_mul
    }

    fn total_resistance(&self) -> f32 {
        self.top_value() + self.bottom_value()
    }

    fn overall_ratio(&self) -> f32 {
        self.top_value() / self.bottom_value()
    }
}

pub struct SternBrocotTree<'a> {
    data: &'a [f32],
}

impl<'a> SternBrocotTree<'a> {
    pub fn new(items: &'a [f32]) -> Self {
        Self { data: items }
    }

    pub fn find_item(&self, target_ratio: f32) -> Vec<ResistorPair> {
        let mut results_vec = Vec::new();
        self.find_item_rec(&mut results_vec, target_ratio, 0, 1, 1, 0, 1.0, 1.0);
        results_vec.reverse();
        results_vec
    }

    fn find_item_rec(
        &self,
        result_vec: &mut Vec<ResistorPair<'a>>,
        target_ratio: f32,
        low_top: usize,
        low_bottom: usize,
        high_top: usize,
        high_bottom: usize,
        mut top_mul: f32,
        mut bottom_mul: f32,
    ) {
        let mut median_top = low_top + high_top;
        let mut median_bottom = low_bottom + high_bottom;

        // If this newly constructed median is out of range, return and don't append.
        if median_top > self.data.len() {
            top_mul *= 10.0;
            median_top = 1;

            if top_mul > MAX_MUL {
                return;
            }
        }

        if median_bottom > self.data.len() {
            bottom_mul *= 10.0;
            median_bottom = 1;

            if bottom_mul > MAX_MUL {
                return;
            }
        }

        // Otherwise, insert this step into the tree
        result_vec.push(ResistorPair::new(
            self.data,
            median_top - 1,
            median_bottom - 1,
            top_mul,
            bottom_mul,
        ));

        let found_number =
            (self.data[median_top - 1] * top_mul) / (self.data[median_bottom - 1] * bottom_mul);

        print!("Found number {found_number} (med is {median_top}/{median_bottom}) (muls are {top_mul}, {bottom_mul}) ");

        if found_number < target_ratio {
            // Found number is smaller than target, need to step to the right
            println!("smaller than target {target_ratio}, stepping larger");
            self.find_item_rec(
                result_vec,
                target_ratio,
                median_top,
                median_bottom,
                high_top,
                high_bottom,
                top_mul,
                bottom_mul,
            )
        } else if found_number > target_ratio {
            // Found number is larger than target, need to step to the left
            println!("larger than target {target_ratio}, stepping smaller");
            self.find_item_rec(
                result_vec,
                target_ratio,
                low_top,
                low_bottom,
                median_top,
                median_bottom,
                top_mul,
                bottom_mul,
            )
        } else {
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let input = [1.0, 2.0, 3.0];

        let tree = SternBrocotTree::new(&input);
        let res: Vec<_> = tree
            .find_item(20.52)
            .iter()
            .map(|pair| pair.overall_ratio())
            .collect();

        let goal = vec![0.5, 0.25];

        assert_eq!(res, goal)
    }
}
