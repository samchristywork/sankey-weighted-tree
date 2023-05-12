pub mod component;
pub mod component_builder;
pub mod parse;
pub mod point;
pub mod render;
pub mod timeline;
pub mod tree_node;

use parse::parse_file;
use render::render_sankey;
use render::render_table;
use std::collections::HashMap;
use tide::Request;
use tide::Response;
use timeline::draw_timeline;
use tree_node::TreeNode;


async fn index(mut _req: Request<()>) -> tide::Result {
    let output = include_str!("template.html");
    let mut res: Response = output.into();
    res.set_content_type("text/html");
    Ok(res)
}

async fn timeline(req: Request<()>) -> tide::Result {
    let query = req.query::<HashMap<String, String>>()?;
    let width = query.get("width").unwrap();
    let height = query.get("height").unwrap();
    Ok(draw_timeline(
        "/home/sam/rofi_time_tracker/log",
        width.parse::<f64>().unwrap(),
        height.parse::<f64>().unwrap(),
    )
    .into())
}

async fn sankey(req: Request<()>) -> tide::Result {
    let query = req.query::<HashMap<String, String>>()?;
    let start_time = query.get("start_time").unwrap();
    let end_time = query.get("end_time").unwrap();
    let width = query.get("width").unwrap();
    let height = query.get("height").unwrap();

    let mut ideal_proportions: HashMap<String, f64> = HashMap::new();
    ideal_proportions.insert("chore".to_string(), 2.4);
    ideal_proportions.insert("entertainment".to_string(), 12.);
    ideal_proportions.insert("finance".to_string(), 6.);
    ideal_proportions.insert("health".to_string(), 24.);
    ideal_proportions.insert("reading".to_string(), 6.);
    ideal_proportions.insert("slop".to_string(), 10.);
    ideal_proportions.insert("social".to_string(), 6.);
    ideal_proportions.insert("task".to_string(), 2.4);
    ideal_proportions.insert("work".to_string(), 30.);
    ideal_proportions.insert("writing".to_string(), 6.);

    let start_time = start_time.parse::<i64>().unwrap();
    let end_time = end_time.parse::<i64>().unwrap();
    let width = width.parse::<f64>().unwrap();
    let height = height.parse::<f64>().unwrap();

    let out = render_table(start_time, end_time, &ideal_proportions)
        + render_sankey(start_time, end_time, width, height, &ideal_proportions).as_str();
    Ok(out.into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/sankey").get(sankey);
    app.at("/timeline").get(timeline);
    app.at("/").get(index);
    app.at("/").serve_dir("static/")?;
    app.listen("127.0.0.1:8723").await?;
    Ok(())
}
