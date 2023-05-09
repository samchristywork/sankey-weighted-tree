pub mod component;
pub mod component_builder;
pub mod parse;
pub mod point;
pub mod timeline;
pub mod tree_node;

use component_builder::ComponentBuilder;
use crate::parse::parse_file;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use tide::Request;
use tide::Response;
use timeline::draw_timeline;
use tree_node::TreeNode;


fn render_tree(tree: &TreeNode, width: f64, height: f64) -> String {
    let mut svg = format!("<svg width='100%' height='100%' xmlns='http://www.w3.org/2000/svg'>\n");

    let mut y = 10.;
    let factor = 1.3*tree.value / height;
    let width = 0.985*width / 3.;
    let mut innercount = 0.;
    let mut middlecount = 0.;
    let mut outercount = 0.;
    let step = 5.;

    let saturation = "50%";
    let lightness = "70%";

    let mut keys: Vec<&String> = tree.children.keys().into_iter().collect();
    keys.sort();
    for key in keys {
        let major = key;
        let x = 10.;

        let value = tree.children[key].value;

        let label = format!("{major}");
        let mut state = DefaultHasher::new();
        label.hash(&mut state);
        let hue = state.finish() % 360;
        svg += ComponentBuilder::new(x, y, x + width - 10., y + outercount)
            .height(value / factor)
            .color(format!("hsl({}, {saturation}, {lightness})", hue).as_str())
            .right_text(label.as_str())
            .data(format!("{label}: {:.2} minutes", value / 60.).as_str())
            .build()
            .draw()
            .as_str();

        let tree = &tree.children[key];
        let mut keys: Vec<&String> = tree.children.keys().into_iter().collect();
        keys.sort();
        for key in keys {
            let minor = key;
            let x = x + width;

            let value = tree.children[key].value;

            let label = format!("{major}.{minor}");
            let mut state = DefaultHasher::new();
            label.hash(&mut state);
            let hue = state.finish() % 360;
            svg += ComponentBuilder::new(x, y + outercount, x + width - 10., y + middlecount)
                .height(value / factor)
                .color(format!("hsl({}, {saturation}, {lightness})", hue).as_str())
                .right_text(label.as_str())
                .data(format!("{label}: {:.2} minutes", value / 60.).as_str())
                .build()
                .draw()
                .as_str();

            let tree = &tree.children[key];
            let mut keys: Vec<&String> = tree.children.keys().into_iter().collect();
            keys.sort();
            for key in keys {
                let activity = key;
                let x = x + width;

                let value = tree.children[key].value;

                let label = format!("{major}.{minor}.{activity}");
                let mut state = DefaultHasher::new();
                label.hash(&mut state);
                let hue = state.finish() % 360;
                svg += ComponentBuilder::new(x, y + middlecount, x + width - 10., y + innercount)
                    .height(value / factor)
                    .color(format!("hsl({}, {saturation}, {lightness})", hue).as_str())
                    .right_text(label.as_str())
                    .data(format!("{label}: {:.2} minutes", value / 60.).as_str())
                    .build()
                    .draw()
                    .as_str();
                y += value / factor;
                innercount += step;
            }
            middlecount += step;
        }
        outercount += step;
    }

    let mut state = DefaultHasher::new();
    "all".hash(&mut state);
    let hue = state.finish() % 360;
    y -= 10.;
    svg += format!("<rect x='0' y='10' width='10' height='{y}' fill='hsl({hue}, {saturation}, {lightness})' />\n").as_str();

    svg + "</svg>\n"
}

fn render_sankey(start_time: &String, end_time: &String, width: &String, height: &String) -> String {

    let tree = parse_file(
        "/home/sam/rofi_time_tracker/log",
        start_time.parse::<i64>().unwrap(),
        end_time.parse::<i64>().unwrap(),
    );

    let svg = render_tree(
        &tree,
        width.parse::<f64>().unwrap(),
        height.parse::<f64>().unwrap(),
    );

    svg
}

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
