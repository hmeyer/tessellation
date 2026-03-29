use crate::bitset::BitSet;

pub const CELL_CONFIGS: &[&[BitSet]] = include!(concat!(env!("OUT_DIR"), "/cell_configs_data.rs"));
