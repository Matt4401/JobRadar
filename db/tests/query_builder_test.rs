// Integration tests for the SQL query builder, exercised through the public
// `build_sql_query` API. Column/table validation is covered indirectly.

use db::orm::query_configs_builder::{
    Filter, Operator, OrderBy, QueryConfigs, QueryType, SortDirection,
};
use db::orm::sql_query_builder::build_sql_query;

fn config(
    query_type: QueryType,
    columns: Vec<String>,
    filters: Option<Vec<Filter>>,
    values: Option<Vec<(String, String)>>,
    order_by: Option<OrderBy>,
    limit: Option<u64>,
) -> QueryConfigs {
    QueryConfigs::new(
        "job_offers".to_string(),
        query_type,
        columns,
        filters,
        values,
        order_by,
        limit,
    )
}

#[test]
fn select_all_columns() {
    let (query, binds) =
        build_sql_query(&config(QueryType::SELECT, vec![], None, None, None, None)).unwrap();
    assert_eq!(query, "SELECT * FROM job_offers");
    assert!(binds.is_empty());
}

#[test]
fn select_specific_columns() {
    let (query, _) = build_sql_query(&config(
        QueryType::SELECT,
        vec!["title".to_string(), "company".to_string()],
        None,
        None,
        None,
        None,
    ))
    .unwrap();
    assert_eq!(query, "SELECT title, company FROM job_offers");
}

#[test]
fn select_with_order_by_and_limit() {
    let (query, _) = build_sql_query(&config(
        QueryType::SELECT,
        vec![],
        None,
        None,
        Some(OrderBy {
            column: "created_at".to_string(),
            direction: SortDirection::Desc,
        }),
        Some(10),
    ))
    .unwrap();
    assert_eq!(
        query,
        "SELECT * FROM job_offers ORDER BY created_at DESC LIMIT 10"
    );
}

#[test]
fn select_with_single_filter_binds_value() {
    let (query, binds) = build_sql_query(&config(
        QueryType::SELECT,
        vec![],
        Some(vec![Filter {
            column: "title".to_string(),
            operator: Operator::Eq,
            value: "Dev".to_string(),
        }]),
        None,
        None,
        None,
    ))
    .unwrap();
    assert_eq!(query, "SELECT * FROM job_offers WHERE title = ?");
    assert_eq!(binds, vec!["Dev".to_string()]);
}

#[test]
fn select_with_multiple_filters_joined_by_and() {
    let (query, binds) = build_sql_query(&config(
        QueryType::SELECT,
        vec![],
        Some(vec![
            Filter {
                column: "title".to_string(),
                operator: Operator::Like,
                value: "%dev%".to_string(),
            },
            Filter {
                column: "location".to_string(),
                operator: Operator::Eq,
                value: "Nantes".to_string(),
            },
        ]),
        None,
        None,
        None,
    ))
    .unwrap();
    assert_eq!(
        query,
        "SELECT * FROM job_offers WHERE title LIKE ? AND location = ?"
    );
    assert_eq!(binds, vec!["%dev%".to_string(), "Nantes".to_string()]);
}

#[test]
fn insert_builds_columns_and_placeholders() {
    let (query, binds) = build_sql_query(&config(
        QueryType::INSERT,
        vec![],
        None,
        Some(vec![
            ("title".to_string(), "Dev".to_string()),
            ("company".to_string(), "ACME".to_string()),
        ]),
        None,
        None,
    ))
    .unwrap();
    assert_eq!(query, "INSERT INTO job_offers (title, company) VALUES (?, ?)");
    assert_eq!(binds, vec!["Dev".to_string(), "ACME".to_string()]);
}

#[test]
fn update_builds_set_clause_and_where() {
    let (query, binds) = build_sql_query(&config(
        QueryType::UPDATE,
        vec![],
        Some(vec![Filter {
            column: "id".to_string(),
            operator: Operator::Eq,
            value: "1".to_string(),
        }]),
        Some(vec![("title".to_string(), "New".to_string())]),
        None,
        None,
    ))
    .unwrap();
    assert_eq!(query, "UPDATE job_offers SET title = ? WHERE id = ?");
    assert_eq!(binds, vec!["New".to_string(), "1".to_string()]);
}

#[test]
fn delete_builds_where_clause() {
    let (query, binds) = build_sql_query(&config(
        QueryType::DELETE,
        vec![],
        Some(vec![Filter {
            column: "id".to_string(),
            operator: Operator::Eq,
            value: "1".to_string(),
        }]),
        None,
        None,
        None,
    ))
    .unwrap();
    assert_eq!(query, "DELETE FROM job_offers WHERE id = ?");
    assert_eq!(binds, vec!["1".to_string()]);
}

#[test]
fn order_by_and_limit_ignored_for_non_select() {
    let (query, _) = build_sql_query(&config(
        QueryType::UPDATE,
        vec![],
        None,
        Some(vec![("title".to_string(), "New".to_string())]),
        Some(OrderBy {
            column: "created_at".to_string(),
            direction: SortDirection::Asc,
        }),
        Some(5),
    ))
    .unwrap();
    assert!(!query.contains("ORDER BY"));
    assert!(!query.contains("LIMIT"));
}

#[test]
fn unknown_table_is_rejected() {
    let cfg = QueryConfigs::new(
        "secret_users".to_string(),
        QueryType::SELECT,
        vec![],
        None,
        None,
        None,
        None,
    );
    assert!(build_sql_query(&cfg).is_err());
}

#[test]
fn unknown_column_is_rejected() {
    let cfg = config(
        QueryType::SELECT,
        vec!["title".to_string(), "password".to_string()],
        None,
        None,
        None,
        None,
    );
    assert!(build_sql_query(&cfg).is_err());
}
