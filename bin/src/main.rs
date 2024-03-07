use std::collections::HashMap;
use elasticsearch::{
    cat::{
        CatAliasesParts,
        // CatIndicesParts,
    },
    http::transport::Transport,
    // indices,
    Elasticsearch,
};
use serde::Deserialize;
use serde_json::Value;
use dsl::{
    sort::{Sort, SortClause, Order},
    search::Search,
    query::{
        bool::Bool,
        r#match::Match,
        Query,
        QueryValue,
        range::{
            Range,
            RangeValue,
        },
    },
    match_clause,
    range_clause,
    sort_clause,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = Transport::single_node("http://192.168.8.170:9200")?;
    let client = Elasticsearch::new(transport);

    let aliases = get_all_aliases(&client)
        .await
        .unwrap_or(Vec::new());

    let mut alias_map: HashMap<String, Vec<String>> = HashMap::new();

    for alias in aliases {
        if let Some(indices) = alias_map.get_mut(&alias.alias) {
            indices.push(alias.index);
        } else {
            alias_map.insert(alias.alias, vec![alias.index]);
        }
    }

    let alias_list: Vec<&String> = alias_map
        .keys()
        .collect();

    // println!("{:?}", alias_map);
    // println!("{:?}", alias_list);

    if let Some(first_index) = alias_list.first() {
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
                                range_clause!(
                                    Range::new("@timestamp")
                                        .gte(RangeValue::Date("now-1h/H".to_owned()))
                                        .lte(RangeValue::Date("now/H".to_owned()))
                                        .format("HH:mm:ss yyyy/MM/DD")
                                )
                            )
                    )
            )
            .sort(
                Sort::new(
                    sort_clause!(
                        SortClause::new("@timestamp")
                            .order(Order::Desc)
                    )
                )
            )
            .build();

        println!("{}", search);

        let r = client
            .search(elasticsearch::SearchParts::Index(&[first_index]))
            .from(0)
            .size(5)
            .body(search)
            // ._source(&["@timestamp"])
            .send()
            .await?;

        // let text = r.text().await?;

        // println!("{}", text);

        let body = r.json::<Value>().await?;
        // println!("{}", body);
        let took = body["took"].as_i64().unwrap();
        println!("Took: {}", took);
        for hit in body["hits"]["hits"].as_array().unwrap() {
            println!("{:?}", hit["_source"]);
        }
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
struct CatAliasRecord {
    alias: String,
    index: String,
}

async fn get_all_aliases(client: &Elasticsearch) -> Option<Vec<CatAliasRecord>> {
    let response = client
        .cat()
        .aliases(CatAliasesParts::Name(&["*", "-.*"]))
        .format("json")
        .send()
        .await;

    if let Ok(response) = response {
        let response_body = response.json::<Vec<CatAliasRecord>>().await;
        if let Ok(aliases) = response_body {
            return Some(aliases)
        }
    }

    None
}
