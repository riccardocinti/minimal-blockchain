pub struct NodeConfig {
    pub mining_enabled: bool,
    pub difficulty: usize,
    pub block_time_delta: u64, // delta that defines whether the current difficulty has to be adjusted
}

impl NodeConfig {
    pub fn new(mining_enabled: bool, difficulty: usize, block_time_delta: u64) -> Self {
        Self {
            mining_enabled,
            difficulty,
            block_time_delta,
        }
    }
}
