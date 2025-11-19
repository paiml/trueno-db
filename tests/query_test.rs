//! Tests for query engine

use trueno_db::query::QueryEngine;

#[test]
fn test_query_engine_parse() {
    // Test basic SQL parsing
    let result = QueryEngine::parse("SELECT * FROM table");
    assert!(result.is_ok(), "Query parsing should succeed");
}

#[test]
fn test_query_engine_parse_complex() {
    // Test complex SQL
    let queries = vec![
        "SELECT id, name FROM users WHERE age > 18",
        "SELECT COUNT(*) FROM events GROUP BY category",
        "SELECT * FROM data ORDER BY score DESC LIMIT 10",
    ];

    for sql in queries {
        let result = QueryEngine::parse(sql);
        assert!(result.is_ok(), "Query parsing should succeed for: {sql}");
    }
}

#[test]
fn test_query_engine_parse_empty() {
    // Test empty query
    let result = QueryEngine::parse("");
    assert!(result.is_ok(), "Empty query should be allowed");
}
