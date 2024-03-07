#[macro_export]
macro_rules! match_clause {
    ($field:expr, $value:expr) => {
        {
            use dsl::query::LeafClause;

            LeafClause::Match(Match::new($field, $value))
        }
    };
}

#[macro_export]
macro_rules! range_clause {
    ($r:expr) => {
        {
            use dsl::query::LeafClause;

            LeafClause::Range($r)
        }
    };
}
