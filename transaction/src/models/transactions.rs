use chrono::{DateTime, Utc};

struct Transaction {
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

// this might change to an enum
struct Category;
