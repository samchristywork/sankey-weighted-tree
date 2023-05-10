pub mod component;
pub mod component_builder;
pub mod parse;
pub mod point;
pub mod render;
pub mod timeline;
pub mod tree_node;

use parse::parse_file;
use render::render_sankey;
use std::collections::HashMap;
use tide::Request;
use tide::Response;
use timeline::draw_timeline;
use tree_node::TreeNode;


async fn index(mut _req: Request<()>) -> tide::Result {
    let output = include_str!("template.html");
    let mut foo: Response = output.into();
    foo.set_content_type("text/html");
    Ok(foo)
}

async fn timeline(req: Request<()>) -> tide::Result {
    let query = req.query::<HashMap<String, String>>()?;
    let width = query.get("width").unwrap();
    Ok(draw_timeline(
        "/home/sam/rofi_time_tracker/log",
        width.parse::<f64>().unwrap(),
    )
    .into())
}

async fn sankey(req: Request<()>) -> tide::Result {
    let query = req.query::<HashMap<String, String>>()?;
    let start_time = query.get("start_time").unwrap();
    let end_time = query.get("end_time").unwrap();
    let width = query.get("width").unwrap();
    let height = query.get("height").unwrap();
    Ok(render_sankey(start_time, end_time, width, height).into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/sankey").get(sankey);
    app.at("/timeline").get(timeline);
    app.at("/").get(index);
    app.listen("127.0.0.1:8725").await?;
    Ok(())
}
