use dsl::{
    sort::Order,
    search::Search,
    query::{
        bool::Bool,
        Query,
        QueryValue,
        range::RangeValue,
    },
    match_clause,
    range_clause,
    sort_clause,
    sort,
};

pub fn benchtest(n: usize) {
    for _ in 0..n {
        {
            let _ = Search::new()
            .query(
                Query::new()
                    .bool(
                        Bool::new()
                            .must(
                                match_clause!(
                                    "event.action", QueryValue::Text("logged-in".to_owned())
                                )
                            )
                            .filter(
                                range_clause!(
                                    "@timestamp",
                                    gte = RangeValue::Date("now-1h/H".to_owned()),
                                    lte = RangeValue::Date("now/H".to_owned()),
                                    format = "HH:mm:ss yyyy/MM/DD"
                                )
                            )
                    )
            )
            .sort(
                sort!(
                    sort_clause!("@timestamp", order = Order::Desc)
                )
            )
            .build();
        }
    }
}
