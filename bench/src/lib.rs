use dsl::{
    clause,
    query::{
        bool::Bool,
        range::RangeValue,
        Query,
        QueryValue,
    },
    search::Search,
    sort::{
        Order,
        Sort,
    },
    sort_clause,
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
                                clause!(
                                    Match,
                                    "event.action",
                                    QueryValue::Text("logged-in".to_owned())
                                )
                            )
                            .filter(
                                clause!(
                                    Range,
                                    "@timestamp",
                                    gte = RangeValue::Date("now-1h/H".to_owned()),
                                    lte = RangeValue::Date("now/H".to_owned()),
                                    format = "HH:mm:ss yyyy/MM/DD"
                                )
                            )
                    )
            )
            .sort(
                Sort::new()
                    .sort(
                        sort_clause!("@timestamp", order = Order::Desc)
                    )
            )
            .build();
        }
    }
}
