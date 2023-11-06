use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct SystemModule {
    block_number: u32,
    nonce: BTreeMap<&'static str, u32>,
    events: Vec<&'static str>,
}

impl SystemModule {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    pub fn inc_nonce(&mut self, who: &'static str) {
        let nonce = self.nonce.get(who).unwrap_or(&0);
        let new_nonce = nonce + 1;
        self.nonce.insert(who, new_nonce);
    }

    pub fn emit_event(&mut self, new_event: &'static str) {
        self.events.push(new_event);
    }
}
