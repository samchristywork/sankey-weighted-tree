pub mod component;
pub mod component_builder;
pub mod point;
pub mod tree_node;

use component_builder::ComponentBuilder;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use tree_node::TreeNode;

fn parse_line(line: &str) -> (i64, String) {
    let mut words = line.split('\t');
    let epoch: i64 = words.next().unwrap().parse().unwrap();
    let tag = words.next().unwrap();

    (epoch, tag.to_string())
}

fn main() {
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let mut activities = Vec::new();

    let mut file = File::open("log").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lines = contents.lines();

    let mut last_line = parse_line(lines.next().unwrap());
    let time_period=60*60*24;

    while let Some(line) = lines.next() {
        let contents = parse_line(line);

        let delta = contents.0 - last_line.0;

        if contents.0 < current_time-time_period || last_line.1 == "health.rest.sleep" {
        } else {
            activities.push((delta, last_line.1));
        }
        last_line = contents;
    }

    let epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let delta = epoch - last_line.0;
    activities.push((delta, last_line.1));

    let mut tree=TreeNode{value:0.0, children:HashMap::new()};

    for activity in &activities {
        let time = activity.0 as f64;
        let major = activity.1.split('.').nth(0).unwrap();
        let minor = activity.1.split('.').nth(1).unwrap();
        let activity = activity.1.split('.').nth(2).unwrap();
        tree.insert(major, minor, activity, time);
        println!("{} | {} | {} | {}", major, minor, activity, time);
    }

    let mut svg = format!(
        "<svg width='100%' height='100%' xmlns='http://www.w3.org/2000/svg'>\n"
    );

    println!("{}", tree.value);
    let mut y=10.;
    let factor=tree.value/700.;
    let width=1870./3.;
    let mut innercount=0.;
    let mut middlecount=0.;
    let mut outercount=0.;

    for key in tree.children.keys() {
        let x=10.;

        let value = tree.children[key].value / factor;
        println!("{} {}", key, value);

        svg += ComponentBuilder::new(x, y, x+width-10., y+outercount)
            .height(value)
            .left_color("red")
            .right_color("grey")
            .right_text(key)
            .data(format!("{}", value).as_str())
            .build()
            .draw()
            .as_str();

        let tree=&tree.children[key];
        for key in tree.children.keys() {
            let x=x+width;

            let value = tree.children[key].value / factor;
            println!("  {} {}", key, value);

            svg += ComponentBuilder::new(x, y+outercount, x+width-10., y+middlecount)
                .height(value)
                .left_color("red")
                .right_color("grey")
                .right_text(key)
                .data(format!("{}", value).as_str())
                .build()
                .draw()
                .as_str();

            let tree=&tree.children[key];
            for key in tree.children.keys() {
                let x=x+width;

                let value = tree.children[key].value / factor;
                println!("    {} {}", key, value);

                svg += ComponentBuilder::new(x, y+middlecount, x+width-10., y+innercount)
                    .height(value)
                    .left_color("red")
                    .right_color("grey")
                    .right_text(key)
                    .data(format!("{}", value).as_str())
                    .build()
                    .draw()
                    .as_str();
                y+=value;
                innercount+=10.;
            }
            middlecount+=10.;
        }
        outercount+=10.;
    }

    svg += "</svg>\n";

    let output = include_str!("template.html");
    std::fs::write("index.html", output.replace("BODY", svg.as_str())).unwrap();
}
