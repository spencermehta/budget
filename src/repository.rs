use chrono::NaiveDate;

use crate::{
    category::{Category, CategoryAssignment},
    transaction::Transaction,
};

pub trait Repository {
    async fn find_transactions(&self, budget_id: String) -> anyhow::Result<Vec<Transaction>>;

    async fn insert_transaction(&self, transaction: Transaction) -> anyhow::Result<()>;

    async fn list_categories(&self, budget_id: String) -> anyhow::Result<Vec<String>>;

    async fn category_spends(
        &self,
        budget_id: String,
        date: NaiveDate,
    ) -> anyhow::Result<Vec<Category>>;

    async fn assign_to_category(&self, category: CategoryAssignment) -> anyhow::Result<()>;

    async fn get_category_assignment(
        &self,
        budget_id: &String,
        category_name: &String,
        date: NaiveDate,
    ) -> anyhow::Result<CategoryAssignment>;
}
