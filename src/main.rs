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
use tide::Request;
use tide::Response;
use tree_node::TreeNode;

struct Row {
    key: String,
    delta: f64,
}

fn parse_line(line: &str) -> (i64, String) {
    let mut words = line.split('\t');
    let epoch: i64 = words.next().unwrap().parse().unwrap();
    let tag = words.next().unwrap();

    (epoch, tag.to_string())
}

fn render_tree(tree: &TreeNode, width: f64, height: f64) -> String {
    let mut svg = format!("<svg width='100%' height='100%' xmlns='http://www.w3.org/2000/svg'>\n");

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

fn parse_file(filename: &str, begin: i64, end: i64) -> TreeNode {
    let mut activities = Vec::new();

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents += format!("{}\tnow.now.now", end).as_str();
    let mut lines = contents.lines();

    let mut last_line = parse_line(lines.next().unwrap());

    while let Some(line) = lines.next() {
        let contents = parse_line(line);
        let mut start_time = last_line.0;
        let mut end_time = contents.0;
        let activity = last_line.1.clone();

        if start_time < begin {
            start_time = begin;
        }

        if end_time > end {
            end_time = end;
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
    }

    tree
}

fn draw_timeline(filename: &str, width: f64) -> String {
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let start_of_first_day = 1672552800;
    let mut current_day = start_of_first_day;

    let mut data: Vec<(Vec<Row>, i64)> = Vec::new();
    loop {
        let tree = parse_file(filename, current_day, current_day + 60 * 60 * 24);

        if tree.children.len() == 0 {
            current_day += 60 * 60 * 24;
            continue;
        }

        let mut keys: Vec<&String> = tree.children.keys().into_iter().collect();
        keys.sort();

        let sum = keys
            .clone()
            .into_iter()
            .map(|key| tree.children[key].value)
            .sum::<f64>();

        data.push((
            keys.into_iter()
                .map(|key| Row {
                    key: key.clone(),
                    delta: tree.children[key].value / sum,
                })
                .collect(),
            current_day,
        ));

        current_day += 60 * 60 * 24;

        if current_day + 60*60*24 > current_time {
            break;
        }
    }

    let saturation = "50%";
    let lightness = "70%";
    let height = 40.;

    let mut x = 0.;
    let x_step = width / data.len() as f64;

    let mut svg = format!(
        "<svg id=timeline width='100%' height='{height}' xmlns='http://www.w3.org/2000/svg'>\n"
    );
    for column in data {
        let mut y = 0.;

        let time = column.1;
        svg += format!(
            "<g class='hover-element' data-tooltip='{time}' onclick='changegraph({time});'>\n"
        )
        .as_str();
        for row in column.0 {
            let mut state = DefaultHasher::new();
            row.key.hash(&mut state);
            let hue = state.finish() % 360;

            let delta = row.delta * height;
            svg += format!("<rect x='{x}' y='{y}' width='{x_step}' height='{delta}' fill='hsl({hue}, {saturation}, {lightness})' />\n").as_str();
            y += delta;
        }
        svg += format!("</g>").as_str();
        x += x_step;
    }
    svg += "<svg><br>\n";

    svg
}

fn draw_sankey(current: &String, period: &String, width: &String, height: &String) -> String {
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        - current.parse::<u64>().unwrap();

    let tree = parse_file(
        "/home/sam/rofi_time_tracker/log",
        current_time as i64,
        period.parse::<i64>().unwrap(),
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

async fn timeline(_req: Request<()>) -> tide::Result {
    Ok(draw_timeline("/home/sam/rofi_time_tracker/log").into())
}

async fn sankey(req: Request<()>) -> tide::Result {
    let query = req.query::<HashMap<String, String>>()?;
    let offset = query.get("offset").unwrap();
    let period = query.get("period").unwrap();
    let width = query.get("width").unwrap();
    let height = query.get("height").unwrap();
    Ok(draw_sankey(offset, period, width, height).into())
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
