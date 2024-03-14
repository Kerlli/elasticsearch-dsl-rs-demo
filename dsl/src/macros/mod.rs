/// Create a clause like Python `**kwargs` function
/// 
/// Usage: clause!([query_name], [field], [query_value]?, [key = value]*)
/// 
/// # Example
/// ```
/// use dsl::{
///     clause,
///     query::{
///         bool::Bool,
///         Query,
///         QueryValue
///     },
///     search::Search,
/// };
/// 
/// let search = Search::new()
///     .query(
///         Query::new()
///             .bool(
///                 Bool::new()
///                     .must(
///                         clause!(
///                             Match,
///                             "event.action",
///                             QueryValue::Text("logged-in".to_owned()),
///                             prefix_length = 3
///                         )
///                     )
///             )
///     )
///     .build();
/// ```
#[macro_export]
macro_rules! clause {
    ($query:ident, $field:expr) => {
        {
            use $crate::query::prelude::*;

            LeafClause::$query(
                &$query::new($field.into())
            )
        }
    };

    ($query:ident, $field:expr $(, $key:ident = $value:expr)*) => {
        {
            use $crate::query::prelude::*;

            LeafClause::$query(
                &$query::new($field.into())
                $(.$key($value))*
            )
        }
    };

    ($query:ident, $field:expr, $query_value:expr) => {
        {
            use $crate::query::prelude::*;

            LeafClause::$query(
                &$query::new($field.into(), $query_value)
            )
        }
    };

    ($query:ident, $field:expr, $query_value:expr $(, $key:ident = $value:expr)*) => {
        {
            use $crate::query::prelude::*;

            LeafClause::$query(
                &$query::new($field.into(), $query_value)
                $(.$key($value))*
            )
        }
    };
}

#[macro_export]
macro_rules! sort {
    ($($clause:expr),*) => {
        {
            use $crate::sort::SortClause;

            let mut clauses: Vec<SortClause> = vec![];

            $(
                clauses.push($clause.clone());
            )*

            clauses
        }
    };
}

/// Create a sort clause like Python `**kwargs` function
/// 
/// # Example
/// ```
/// use dsl::search::Search;
/// use dsl::sort::{Order, Mode, SortClause};
/// use dsl::{sort, sort_clause};
/// 
/// let search = Search::new()
///     .sort(
///         sort!(
///             sort_clause!("foo", order = Order::Asc, mode = Mode::Min),
///             sort_clause!("bar")
///         )
///     );
/// ```
#[macro_export]
macro_rules! sort_clause {
    ($field:expr) => {
        {
            use $crate::sort::SortClause;

            SortClause::new($field)
        }
    };

    ($field:expr $(, $key:ident = $value:expr)*) => {
        {
            use $crate::sort::SortClause;

            SortClause::new($field)
                $(.$key($value))*
        }
    };
}
