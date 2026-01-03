pub struct NodeConfig {
    pub mining_enabled: bool,
    pub difficulty: usize, //fixed property before the difficulty adjustment step implementation
}

impl NodeConfig {
    pub fn new(mining_enabled: bool) -> Self {
        Self {
            mining_enabled,
            difficulty: 2,
        }
    }
}
