pub mod component;
pub mod component_builder;
pub mod parse;
pub mod point;
pub mod render;
pub mod timeline;
pub mod tree_node;

use parse::parse_file;
use render::render_band;
use render::render_sankey;
use render::render_table;
use std::collections::HashMap;
use std::io::Read;
use tide::Request;
use tide::Response;
use timeline::draw_timeline;
use tree_node::TreeNode;

fn get_ideal_proportions() -> HashMap<String, f64> {
    let mut file = std::fs::File::open("/home/sam/rofi_time_tracker/ideals").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut ideal_proportions: HashMap<String, f64> = HashMap::new();
    for line in contents.lines() {
        let mut split = line.split(" ");
        let name = split.next().unwrap().to_string();
        let proportion = split.next().unwrap().parse::<f64>().unwrap();
        ideal_proportions.insert(name, proportion);
    }

    ideal_proportions
}

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

    let ideal_proportions = get_ideal_proportions();

    Ok(draw_timeline(
        &ideal_proportions,
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

    let start_time = start_time.parse::<u64>().unwrap();
    let end_time = end_time.parse::<u64>().unwrap();
    let width = width.parse::<f64>().unwrap();
    let height = height.parse::<f64>().unwrap();

    let ideal_proportions = get_ideal_proportions();

    let out = render_table(start_time, end_time, &ideal_proportions)
        + render_sankey(start_time, end_time, width, height, &ideal_proportions).as_str();
    Ok(out.into())
}

async fn band(req: Request<()>) -> tide::Result {
    let query = req.query::<HashMap<String, String>>()?;
    let start_time = query.get("start_time").unwrap();
    let end_time = query.get("end_time").unwrap();
    let width = query.get("width").unwrap();
    let height = query.get("height").unwrap();

    let start_time = start_time.parse::<u64>().unwrap();
    let end_time = end_time.parse::<u64>().unwrap();
    let width = width.parse::<f64>().unwrap();
    let height = height.parse::<f64>().unwrap();

    let out = render_band(start_time, end_time, width, height);
    Ok(out.into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/sankey").get(sankey);
    app.at("/band").get(band);
    app.at("/timeline").get(timeline);
    app.at("/").get(index);
    app.at("/").serve_dir("static/")?;
    app.listen("0.0.0.0:8723").await?;
    Ok(())
}
