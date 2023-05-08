pub mod component;
pub mod component_builder;
pub mod point;
pub mod tree_node;

use component_builder::ComponentBuilder;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;
use tide::http::mime;
use tide::prelude::*;
use tide::Request;
use tide::Response;
use tree_node::TreeNode;

fn parse_line(line: &str) -> (i64, String) {
    let mut words = line.split('\t');
    let epoch: i64 = words.next().unwrap().parse().unwrap();
    let tag = words.next().unwrap();

    (epoch, tag.to_string())
}

fn render_tree(tree: &TreeNode, id: &str) -> String {
    let mut svg = format!("<svg id={id} style='display: none' width='100%' height='100%' xmlns='http://www.w3.org/2000/svg'>\n");

    let mut y = 10.;
    let factor = tree.value / 700.;
    let width = 1870. / 3.;
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
        println!("{} {}", key, value);

        let label = format!("{major}");
        let mut state = DefaultHasher::new();
        label.hash(&mut state);
        let hue = state.finish() % 360;
        svg += ComponentBuilder::new(x, y, x + width - 10., y + outercount)
            .height(value/factor)
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
            println!("  {} {}", key, value);

            let label = format!("{major}.{minor}");
            let mut state = DefaultHasher::new();
            label.hash(&mut state);
            let hue = state.finish() % 360;
            svg += ComponentBuilder::new(x, y + outercount, x + width - 10., y + middlecount)
                .height(value/factor)
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
                println!("    {} {}", key, value);

                let label = format!("{major}.{minor}.{activity}");
                let mut state = DefaultHasher::new();
                label.hash(&mut state);
                let hue = state.finish() % 360;
                svg += ComponentBuilder::new(x, y + middlecount, x + width - 10., y + innercount)
                    .height(value/factor)
                    .color(format!("hsl({}, {saturation}, {lightness})", hue).as_str())
                    .right_text(label.as_str())
                    .data(format!("{label}: {:.2} minutes", value / 60.).as_str())
                    .build()
                    .draw()
                    .as_str();
                y += value/factor;
                innercount += step;
            }
            middlecount += step;
        }
        outercount += step;
    }

    svg + "</svg>\n"
}

fn parse_file(filename: &str, current_time: i64, time_period: i64) -> TreeNode {
    let mut activities = Vec::new();

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents += format!("{}\tnow.now.now", current_time).as_str();
    let mut lines = contents.lines();

    let mut last_line = parse_line(lines.next().unwrap());

    while let Some(line) = lines.next() {
        let contents = parse_line(line);
        let mut start_time = last_line.0;
        let mut end_time = contents.0;
        let activity = last_line.1.clone();

        if start_time < current_time - time_period {
            start_time = current_time - time_period;
        }

        if end_time > current_time {
            end_time = current_time;
        }

        let delta = end_time - start_time;

        if delta > 0 && activity != "health.rest.sleep" {
            activities.push((delta, activity));
        }

        last_line = contents;
    }

    let mut tree = TreeNode {
        value: 0.0,
        children: HashMap::new(),
    };

    for activity in &activities {
        let time = activity.0 as f64;
        let major = activity.1.split('.').nth(0).unwrap();
        let minor = activity.1.split('.').nth(1).unwrap();
        let activity = activity.1.split('.').nth(2).unwrap();
        tree.insert(major, minor, activity, time);
        println!("{} | {} | {} | {}", major, minor, activity, time);
    }

    tree
}

fn main() {
    let mut svg=String::new();

    let tree = parse_file("/home/sam/rofi_time_tracker/log", 60*60*24*365);
    svg+=render_tree(&tree, "graph-yearly").as_str();

    let tree = parse_file("/home/sam/rofi_time_tracker/log", 60*60*24*30);
    svg+=render_tree(&tree, "graph-monthly").as_str();

    let tree = parse_file("/home/sam/rofi_time_tracker/log", 60*60*24*7);
    svg+=render_tree(&tree, "graph-weekly").as_str();

    let tree = parse_file("/home/sam/rofi_time_tracker/log", 60*60*24);
    svg+=render_tree(&tree, "graph-24-hours").as_str();

    let tree = parse_file("/home/sam/rofi_time_tracker/log", 60*60*12);
    svg+=render_tree(&tree, "graph-12-hours").as_str();

    let tree = parse_file("/home/sam/rofi_time_tracker/log", 60*60*6);
    svg+=render_tree(&tree, "graph-6-hours").as_str();

    let tree = parse_file("/home/sam/rofi_time_tracker/log", 60*60*1);
    svg+=render_tree(&tree, "graph-1-hours").as_str();

    let output = include_str!("template.html");
    output.replace("BODY", svg.as_str())
}

async fn index(mut _req: Request<()>) -> tide::Result {
    let output = include_str!("template.html").replace("BODY", app().as_str());
    let mut foo: Response = output.into();
    foo.set_content_type("text/html");
    Ok(foo)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(index);
    app.listen("127.0.0.1:8725").await?;
    Ok(())
}
