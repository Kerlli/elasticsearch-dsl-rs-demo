### A toy project for Elasticsearch DSL investigation

---

**Note**: This project is designed to work with Elasticsearch `v7.14` and is intended solely for personal exploration. It does not guarantee compatibility or suitability for production environments.

---

### Usage

```rust
use elasticsearch::cat::CatAliasesParts;
use dsl::{
    match_clause,
    query::{
        bool::Bool,
        r#match::Match,
        range::RangeValue,
        Query,
        QueryValue
    },
    range,
    search::Search,
    sort::Order,
    sort_clause,
    sort,
};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let search = Search::new()
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
                            range!(
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

    // search body
    println!("{}", search);

    let r = client
        .search(elasticsearch::SearchParts::Index(&["my_index"]))
        .from(0)
        .size(5)
        .body(search)
        .send()
        .await?;

    let body = r.json::<Value>().await?;
    let took = body["took"].as_i64().unwrap();

    for hit in body["hits"]["hits"].as_array().unwrap() {
        println!("{:?}", hit["_source"]);
    }

    Ok(())
}
```

---

### LICENSE

[MIT](LICENSE)
