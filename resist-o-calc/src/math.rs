use lazy_static::lazy_static;

use crate::{
    dynamic_tree::SternBrocotTree,
    error::{LibError, Result},
    tree::{process_slice, DynamicTreeNode},
};

lazy_static! {
    static ref E3: SternBrocotTree<'static> = SternBrocotTree::new(&[1.0, 2.2, 4.7]);
    static ref E6: SternBrocotTree<'static> = SternBrocotTree::new(&[1.0, 1.5, 2.2, 3.3, 4.7, 6.8]);
    static ref E12: SternBrocotTree<'static> =
        SternBrocotTree::new(&[1.0, 1.2, 1.5, 1.8, 2.2, 2.7, 3.3, 3.9, 4.7, 5.6, 6.8, 8.2]);
}

pub enum ResistorSets {
    E3,
    E6,
    E12,
    E24,
}

impl ResistorSets {
    fn value(&self) -> &SternBrocotTree<'static> {
        match self {
            ResistorSets::E3 => &E3,
            ResistorSets::E6 => &E6,
            ResistorSets::E12 => &E12,
            ResistorSets::E24 => todo!(),
        }
    }
}

pub fn find_by_voltage(v1: f64, v2: f64) -> Result<()> {
    if !v1.is_normal() || v1 < 0.0 {
        return Err(LibError::InvalidVoltage(v1));
    }

    if !v2.is_normal() || v2 < 0.0 {
        return Err(LibError::InvalidVoltage(v2));
    }

    let ratio = v1 / v2;

    Ok(())
}
