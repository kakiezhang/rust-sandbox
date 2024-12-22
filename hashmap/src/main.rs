use std::collections::HashMap;

type MatesPost = HashMap<String, HashMap<String, String>>;

fn main() {
    let mut mates_post: MatesPost = HashMap::new();
    let date = "20230805".to_string();
    let mate_name = "Bob";
    let content = "hello";

    mates_post
        .entry(date.clone())
        .or_insert_with(HashMap::new)
        .insert(mate_name.to_string(), content.to_string());

    let mate_name = "Mary";
    let content = "world";
    mates_post
        .entry(date.clone())
        .or_insert_with(HashMap::new)
        .insert(mate_name.to_string(), content.to_string());

    let mate_name = "Bob";
    let content = "hi";
    mates_post
        .entry(date.clone())
        .or_insert_with(HashMap::new)
        .insert(mate_name.to_string(), content.to_string());

    println!("{:?}", mates_post);
}
