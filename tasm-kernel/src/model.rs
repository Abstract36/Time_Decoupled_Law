use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

/// Time is defined as a discrete Slot number.
pub type Slot = u64;

/// Unique Identifier for an Intent (SHA-256 Hash).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IntentId(pub String);

/// An Intent is a commitment to perform an act by a deadline.
/// If the act is not observed by the deadline, it becomes an Absence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub id: IntentId,
    pub creator: String, // Account ID
    pub description: String,
    pub deadline: Slot,
    pub collateral: u64,
}

impl Intent {
    pub fn new(creator: &str, description: &str, deadline: Slot, collateral: u64) -> Self {
        let mut intent = Self {
            id: IntentId("pending".to_string()),
            creator: creator.to_string(),
            description: description.to_string(),
            deadline,
            collateral,
        };
        intent.id = intent.calculate_id();
        intent
    }

    pub fn calculate_id(&self) -> IntentId {
        let mut hasher = Sha256::new();
        hasher.update(self.creator.as_bytes());
        hasher.update(self.description.as_bytes());
        hasher.update(self.deadline.to_le_bytes());
        hasher.update(self.collateral.to_le_bytes());
        let result = hasher.finalize();
        IntentId(hex::encode(result))
    }
}

/// An Absence is the irrefutable fact that an Intent was not fulfilled.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Absence {
    pub intent_id: IntentId,
    pub declared_at: Slot,
}
