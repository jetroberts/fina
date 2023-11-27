use chrono::{DateTime, Utc};

use crate::services::transaction_service::GetId;

pub struct Transaction {
    id: String,
    amount: f32,
    // think of a better name
    transaction_type: String,
    timestamp: DateTime<Utc>,
    user: String,
    category: Option<Category>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl Transaction {
    pub fn new(
        amount: f32,
        transaction_type: String,
        timestamp: DateTime<Utc>,
        user: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            amount,
            transaction_type,
            timestamp,
            user,
            category: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }
}

impl GetId for Transaction {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl ToString for Transaction {
    fn to_string(&self) -> String {
        let cat = match &self.category {
            Some(c) => c.to_string(),
            None => "None".to_string(),
        };

        format!(
            "id: {}, amount: {}, transaction_type: {}, timestamp: {}, user: {}, category: {}, created_at: {}, updated_at: {}, deleted_at: {}",
            self.id,
            self.amount,
            self.transaction_type,
            self.timestamp,
            self.user,
            cat,
            self.created_at,
            self.updated_at,
            self.deleted_at.unwrap_or(Utc::now())
        )
    }
}

// this might change to an enum

struct Category;

impl ToString for Category {
    fn to_string(&self) -> String {
        "category".to_string()
    }
}
