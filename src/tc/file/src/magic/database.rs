pub struct MagicRule {
    pub offset: usize,
    pub bytes: Vec<u8>,
    pub description: String,
}

pub fn load_magic_rules() -> Vec<MagicRule> {
    // 假定加载本地 magic 规则，实际可解析 magic.mgc
    vec![
        MagicRule { offset: 0, bytes: vec![0x7F, b'E', b'L', b'F'], description: "ELF executable".to_string() },
        MagicRule { offset: 0, bytes: vec![0xFF, 0xD8, 0xFF], description: "JPEG image".to_string() },
        // 更多规则
    ]
}