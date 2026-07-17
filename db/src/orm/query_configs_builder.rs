use crate::models::job_offer::JobOffer;
use crate::models::migration_history::MigrationsHistory;
use crate::models::Table;
use std::collections::HashMap;
use std::sync::LazyLock;

pub enum QueryType {
    SELECT,
    INSERT,
    #[allow(dead_code)]
    UPDATE,
    #[allow(dead_code)]
    DELETE,
}

#[allow(dead_code)]
pub enum Operator {
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    Like,
    In,
}

pub struct Filter {
    pub column: String,
    pub operator: Operator,
    pub value: String,
}

#[allow(dead_code)]
pub enum SortDirection {
    Asc,
    Desc,
}

/// Ordering clause of a SELECT: which column to sort on and in which direction.
pub struct OrderBy {
    pub column: String,
    pub direction: SortDirection,
}

#[allow(dead_code)]
pub struct QueryConfigs {
    pub(crate) struct_name: String,
    pub(crate) table_name: String,
    pub(crate) query_type: QueryType,
    pub(crate) columns: Vec<String>,
    pub(crate) filters: Option<Vec<Filter>>,
    pub(crate) values: Option<Vec<(String, String)>>,
    pub(crate) order_by: Option<OrderBy>,
    pub(crate) limit: Option<u64>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TableModel {
    Job(JobOffer),
    Migration(MigrationsHistory),
}

static STRUCT_FROM_TABLE: LazyLock<HashMap<&'static str, TableModel>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(JobOffer::TABLE, TableModel::Job(JobOffer::default()));
    m.insert(
        MigrationsHistory::TABLE,
        TableModel::Migration(MigrationsHistory::default()),
    );

    m
});

impl QueryConfigs {
    pub fn new(
        table_name: String,
        query_type: QueryType,
        columns: Vec<String>,
        filters: Option<Vec<Filter>>,
        values: Option<Vec<(String, String)>>,
        order_by: Option<OrderBy>,
        limit: Option<u64>,
    ) -> Self {
        let struct_name = QueryConfigs::get_struct_from_table(&table_name);

        QueryConfigs {
            struct_name,
            table_name,
            query_type,
            columns,
            filters,
            values,
            order_by,
            limit,
        }
    }

    fn get_struct_from_table(table_name: &str) -> String {
        let mut struct_name = String::new();
        if let Some(table_model) = STRUCT_FROM_TABLE.get(table_name) {
            match table_model {
                TableModel::Job(_) => struct_name = "JobOffer".to_string(),
                TableModel::Migration(_) => struct_name = "MigrationsHistory".to_string(),
            }
        }
        struct_name
    }
}
