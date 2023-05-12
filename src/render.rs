use crate::component_builder::ComponentBuilder;
use crate::parse::parse_file;
use crate::tree_node::TreeNode;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

fn format_time(timestamp: i64) -> String {
    let hours = timestamp / 3600;
    let minutes = (timestamp % 3600) / 60;
    let seconds = timestamp % 60;

    format!("{}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn render_tree(
    tree: &TreeNode,
    width: f64,
    height: f64,
    highlight: [String; 3],
    ideal_proportions: &HashMap<String, f64>,
) -> String {
    let mut svg = format!("<svg width='100%' height='100%' xmlns='http://www.w3.org/2000/svg'>\n");

    let mut y = 10.;
    let factor = 1.9 * tree.value / height;
    let width = 0.65 * width / 3.;
    let mut innercount = 0.;
    let mut middlecount = 0.;
    let mut outercount = 0.;
    let step = 10.;
    let font_size = 1.2 * height / 100.;

    let saturation = "30%";
    let lightness = "50%";

    let total_day_length = tree.value;

    let mut keys: Vec<&String> = tree.children.keys().into_iter().collect();
    keys.sort();
    for key in keys {
        let major = key;
        let x = 10.;

        let value = tree.children[key].value;

        let color = match major == highlight[0].as_str() {
            false => "#444",
            true => "#222",
        };

        let label = format!("{major}");
        let mut state = DefaultHasher::new();
        label.hash(&mut state);
        let hue = state.finish() % 360;
        svg += ComponentBuilder::new(x, y, x + width - 10., y + outercount)
            .height(value / factor)
            .color(format!("hsl({}, {saturation}, {lightness})", hue).as_str())
            .body_color(color)
            .right_text(label.as_str())
            .font_size(font_size)
            .data(
                format!(
                    "{label}: {} ({:.3}%)",
                    format_time(value as i64),
                    value / total_day_length * 100.
                )
                .as_str(),
            )
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

            let color = match major == highlight[0].as_str() && minor == highlight[1].as_str() {
                false => "#444",
                true => "#222",
            };

            let label = format!("{major}.{minor}");
            let mut state = DefaultHasher::new();
            label.hash(&mut state);
            svg += ComponentBuilder::new(x, y + outercount, x + width - 10., y + middlecount)
                .height(value / factor)
                .color(format!("hsl({}, {saturation}, {lightness})", hue).as_str())
                .body_color(color)
                .right_text(label.as_str())
                .font_size(font_size)
                .data(
                    format!(
                        "{label}: {} ({:.3}%)",
                        format_time(value as i64),
                        value / total_day_length * 100.
                    )
                    .as_str(),
                )
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

                let color = match major == highlight[0].as_str()
                    && minor == highlight[1].as_str()
                    && activity == highlight[2].as_str()
                {
                    false => "#444",
                    true => "#222",
                };

                let label = format!("{major}.{minor}.{activity}");
                let mut state = DefaultHasher::new();
                label.hash(&mut state);
                svg += ComponentBuilder::new(x, y + middlecount, x + width - 10., y + innercount)
                    .height(value / factor)
                    .color(format!("hsl({}, {saturation}, {lightness})", hue).as_str())
                    .body_color(color)
                    .right_text(label.as_str())
                    .font_size(font_size)
                    .data(
                        format!(
                            "{label}: {} ({:.3}%)",
                            format_time(value as i64),
                            value / total_day_length * 100.
                        )
                        .as_str(),
                    )
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

    let mut current = 10.;
    let range = y - 10.;
    let domain = ideal_proportions.iter().fold(0., |acc, x| acc + x.1);

    for x in ideal_proportions {
        let mut state = DefaultHasher::new();
        x.0.hash(&mut state);
        let hue = state.finish() % 360;
        let height = x.1 / domain * range;
        let label = &x.0;
        svg += format!("<rect x='0' y='{current}' width='10' height='{height}' class='hover-element' data-tooltip='{label} ({:.3}%)' fill='hsl({hue}, {saturation}, {lightness})' />\n", x.1 / domain * 100.).as_str();
        current += height;
    }

    svg + "</svg>\n"
}

pub fn render_table(
    start_timestamp: i64,
    end_timestamp: i64,
    ideal_proportions: &HashMap<String, f64>,
) -> String {
    let (tree, _) = parse_file(
        "/home/sam/rofi_time_tracker/log",
        start_timestamp,
        end_timestamp,
    );

    let mut out = String::from("<span class='stats-container'>");

    let mut keys: Vec<&String> = tree.children.keys().into_iter().collect();
    keys.sort();

    let time_domain = keys
        .iter()
        .fold(0., |acc, x| tree.children[x.as_str()].value + acc);
    let ideal_domain = ideal_proportions.iter().fold(0., |acc, x| acc + x.1);

    for key in keys {
        let mut ideal_value = 0.;
        for ideal in ideal_proportions {
            if key == ideal.0.as_str() {
                ideal_value = 100. * ideal.1 / ideal_domain;
            }
        }
        let actual_value = 100. * tree.children[key].value / time_domain;

        let color = match actual_value > ideal_value {
            false => "red",
            true => "green",
        };

        out += format!("<span style='color: {}'> {}</span>", color, key,).as_str();
        out += format!("<span style='color: {}'>{:.3}%</span>", color, actual_value,).as_str();
        out += format!("<span style='color: {}'>{:.3}%</span>", color, ideal_value).as_str();
    }

    out + "</span>"
}

pub fn render_sankey(
    start_timestamp: i64,
    end_timestamp: i64,
    width: f64,
    height: f64,
    ideal_proportions: &HashMap<String, f64>,
) -> String {
    let (tree, current) = parse_file(
        "/home/sam/rofi_time_tracker/log",
        start_timestamp,
        end_timestamp,
    );

    let svg = render_tree(&tree, width, height, current, ideal_proportions);

    svg
}
