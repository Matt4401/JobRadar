use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;

pub fn validate_syntax(query: &str) -> Result<(), String> {
    let dialect = MySqlDialect {};
    Parser::parse_sql(&dialect, query)
        .map(|_| ())
        .map_err(|e| format!("Invalid SQL syntax : {e}"))
}
