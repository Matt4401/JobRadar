use crate::connection::Database;
use crate::orm::query_configs_builder::{Operator, QueryConfigs, QueryType, SortDirection};
use crate::orm::security::secure_request::{
    detect_suspicious_patterns, validate_column, validate_table,
};
use crate::orm::security::verify_syntax::validate_syntax;

pub async fn parse_and_execute_query(
    db: &Database,
    config: QueryConfigs,
) -> Result<(), sqlx::Error> {
    let (query, binds) = build_sql_query(&config).map_err(sqlx::Error::Protocol)?;
    validate_syntax(&query).map_err(sqlx::Error::Protocol)?;
    detect_suspicious_patterns(&query).map_err(sqlx::Error::Protocol)?;

    let execution_result = db.execute_query(&query, &binds).await;
    match &execution_result {
        Ok(()) => println!("The request was executed successfully."),
        Err(e) => eprintln!("Error during request execution : {}", e),
    }
    execution_result
}

/// Returns the SQL string together with the ordered list of values to bind.
pub(crate) fn build_sql_query(config: &QueryConfigs) -> Result<(String, Vec<String>), String> {
    validate_table(&config.table_name)?;

    let mut binds: Vec<String> = Vec::new();
    let mut query = String::new();

    query = add_action(query, config)?;
    query = add_cols_or_values(query, config, &mut binds)?;
    query = add_filters(query, config, &mut binds)?;
    query = add_order_by(query, config)?;
    query = add_limit(query, config);

    Ok((query, binds))
}

fn add_action(mut query: String, config: &QueryConfigs) -> Result<String, String> {
    match config.query_type {
        QueryType::SELECT => {
            let cols = if config.columns.is_empty() {
                "*".to_string()
            } else {
                for column in &config.columns {
                    validate_column(&config.table_name, column)?;
                }
                config.columns.join(", ")
            };
            query.push_str(&format!("SELECT {} FROM {}", cols, config.table_name));
        }
        QueryType::INSERT => {
            query.push_str(&format!("INSERT INTO {}", config.table_name));
        }
        QueryType::UPDATE => {
            query.push_str(&format!("UPDATE {}", config.table_name));
        }
        QueryType::DELETE => {
            query.push_str(&format!("DELETE FROM {}", config.table_name));
        }
    }
    Ok(query)
}

fn add_cols_or_values(
    mut query: String,
    config: &QueryConfigs,
    binds: &mut Vec<String>,
) -> Result<String, String> {
    match config.query_type {
        QueryType::INSERT => {
            if let Some(values) = &config.values {
                for (column, _) in values {
                    validate_column(&config.table_name, column)?;
                }

                let columns: Vec<&str> = values.iter().map(|(c, _)| c.as_str()).collect();
                let placeholders: Vec<&str> = values.iter().map(|_| "?").collect();

                for (_, value) in values {
                    binds.push(value.clone());
                }

                query.push_str(&format!(
                    " ({}) VALUES ({})",
                    columns.join(", "),
                    placeholders.join(", ")
                ));
            }
        }
        QueryType::UPDATE => {
            if let Some(values) = &config.values {
                let mut set_clause: Vec<String> = Vec::with_capacity(values.len());
                for (column, value) in values {
                    validate_column(&config.table_name, column)?;
                    set_clause.push(format!("{column} = ?"));
                    binds.push(value.clone());
                }

                query.push_str(&format!(" SET {}", set_clause.join(", ")));
            }
        }
        _ => {}
    }
    Ok(query)
}

fn add_filters(
    mut query: String,
    config: &QueryConfigs,
    binds: &mut Vec<String>,
) -> Result<String, String> {
    let Some(filters) = &config.filters else {
        return Ok(query);
    };
    if filters.is_empty() {
        return Ok(query);
    }

    let mut clauses: Vec<String> = Vec::with_capacity(filters.len());
    for f in filters {
        validate_column(&config.table_name, &f.column)?;
        clauses.push(format!("{} {} ?", f.column, operator_to_sql(&f.operator)));
        binds.push(f.value.clone());
    }

    query.push_str(&format!(" WHERE {}", clauses.join(" AND ")));
    Ok(query)
}

fn operator_to_sql(op: &Operator) -> &'static str {
    match op {
        Operator::Eq => "=",
        Operator::Neq => "!=",
        Operator::Gt => ">",
        Operator::Lt => "<",
        Operator::Gte => ">=",
        Operator::Lte => "<=",
        Operator::Like => "LIKE",
        Operator::In => "IN",
    }
}

// only for select queries
fn add_order_by(mut query: String, config: &QueryConfigs) -> Result<String, String> {
    if !matches!(config.query_type, QueryType::SELECT) {
        return Ok(query);
    }
    let Some(order_by) = &config.order_by else {
        return Ok(query);
    };

    validate_column(&config.table_name, &order_by.column)?;
    let direction_str = match order_by.direction {
        SortDirection::Asc => "ASC",
        SortDirection::Desc => "DESC",
    };

    query.push_str(&format!(" ORDER BY {} {}", order_by.column, direction_str));
    Ok(query)
}

// only for select queries
fn add_limit(mut query: String, config: &QueryConfigs) -> String {
    if !matches!(config.query_type, QueryType::SELECT) {
        return query;
    }
    if let Some(limit) = config.limit {
        query.push_str(&format!(" LIMIT {}", limit));
    }
    query
}
