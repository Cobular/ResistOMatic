use std::fmt::Display;

use crate::{types::ResistorPair, utils::float_to_val};

static NUM_DECADES: usize = 3;

#[cfg(debug_assertions)]
impl<'a> Display for ResistorPair<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.top_value(), self.bottom_value()))
    }
}

pub struct SternBrocotTree<'a> {
    data: &'a [f64],
}

impl<'a> SternBrocotTree<'a> {
    pub fn new(items: &'a [f64]) -> Self {
        Self { data: items }
    }

    pub fn find_best_matches(&self, target_ratio: f64, num_matches: usize) -> Vec<ResistorPair> {
        let mut items = self.find_item(target_ratio);
        items.sort_by(|a, b| {
            ((a.overall_ratio() - target_ratio).abs())
                .partial_cmp(&(b.overall_ratio() - target_ratio).abs())
                .unwrap()
                .then(
                    a.total_resistance()
                        .partial_cmp(&b.total_resistance())
                        .unwrap(),
                )
        });
        items.into_iter().take(num_matches).collect()
    }

    pub fn find_item(&self, target_ratio: f64) -> Vec<ResistorPair> {
        let mut results_vec = Vec::new();
        self.find_item_rec(&mut results_vec, target_ratio, 0, 0);
        results_vec.reverse();
        results_vec
    }

    fn find_item_rec(
        &self,
        result_vec: &mut Vec<ResistorPair<'a>>,
        target_ratio: f64,
        top: usize,
        bottom: usize,
    ) {
        if top / self.data.len() > NUM_DECADES || bottom / self.data.len() > NUM_DECADES {
            #[cfg(debug_assertions)]
            println!("early returning");
            return;
        }

        // Otherwise, insert this step into the tree
        result_vec.push(ResistorPair::new(self.data, top, bottom));

        let found_number = (float_to_val(self.data, top)) / float_to_val(self.data, bottom);

        #[cfg(debug_assertions)]
        print!(
            "Found number {found_number} (med is {top}/{bottom}, actual val is {}/{})",
            float_to_val(self.data, top),
            float_to_val(self.data, bottom)
        );

        if found_number < target_ratio {
            // Found number is smaller than target, need to step to the right
            #[cfg(debug_assertions)]
            println!("smaller than target {target_ratio}, stepping larger");
            self.find_item_rec(result_vec, target_ratio, top + 1, bottom)
        } else if found_number > target_ratio {
            // Found number is larger than target, need to step to the left
            #[cfg(debug_assertions)]
            println!("larger than target {target_ratio}, stepping smaller");
            self.find_item_rec(result_vec, target_ratio, top, bottom + 1)
        } else {
            #[cfg(debug_assertions)]
            println!("went off the else case");
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_f2v_simple() {
        assert_eq!(float_to_val(&[1.0, 2.0, 3.0], 0), 1.0);
        assert_eq!(float_to_val(&[1.0, 2.0, 3.0], 2), 3.0);
        assert_eq!(float_to_val(&[1.0, 2.0, 3.0], 3), 10.0);
        assert_eq!(float_to_val(&[1.0, 2.0, 3.0], 5), 30.0);
        assert_eq!(float_to_val(&[1.0, 2.0, 3.0], 10), 2000.0);
    }

    #[test]
    fn test_e3() {
        let input = [1.0, 2.2, 4.7];

        let tree = SternBrocotTree::new(&input);

        let res: Vec<_> = tree.find_best_matches(0.3, 5);
        assert_eq!(res[0].overall_ratio(), 0.22000000000000003);
        assert_eq!(res[0].top_value(), 2.2);
        assert_eq!(res[0].bottom_value(), 10.0);

        let res: Vec<_> = tree.find_best_matches(5.0, 5);
        assert_eq!(res[0].overall_ratio(), 4.7);
        assert_eq!(res[0].top_value(), 4.7);
    }
}
