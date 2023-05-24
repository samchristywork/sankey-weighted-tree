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
    let width = 0.47 * width / 3.;
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

    let mut keys: Vec<&String> = ideal_proportions.keys().into_iter().collect();
    keys.sort();

    for key in keys {
        let value = ideal_proportions[key];

        let mut state = DefaultHasher::new();
        key.hash(&mut state);
        let hue = state.finish() % 360;
        let height = value / domain * range;
        let label = &key;
        svg += format!("<rect x='0' y='{current}' width='10' height='{height}' class='hover-element' data-tooltip='{label} ({:.3}%)' fill='hsl({hue}, {saturation}, {lightness})' />\n", value / domain * 100.).as_str();
        current += height;
    }

    svg + "</svg>\n"
}

pub fn get_points(tree: &TreeNode, ideal_proportions: &HashMap<String, f64>) -> f64 {
    let mut keys: Vec<&String> = tree.children.keys().into_iter().collect();
    keys.sort();

    let time_domain = keys
        .iter()
        .fold(0., |acc, x| tree.children[x.as_str()].value + acc);
    let ideal_domain = ideal_proportions.iter().fold(0., |acc, x| acc + x.1);

    let mut points = 0.;
    for key in keys {
        let mut ideal_value = 0.;

        let value = match ideal_proportions.get(key.as_str()) {
            Some(x) => x,
            None => continue,
        };

        ideal_value += 100. * value / ideal_domain;
        let actual_value = 100. * tree.children[key].value / time_domain;

        match actual_value > ideal_value {
            false => {
                points += actual_value;
            }
            true => {
                points += ideal_value;
            }
        };
    }

    points += ideal_proportions["slop"];

    points
}

pub fn render_table(
    start_timestamp: i64,
    end_timestamp: i64,
    ideal_proportions: &HashMap<String, f64>,
) -> String {
    let (tree, current, _) = parse_file(
        "/home/sam/rofi_time_tracker/log",
        start_timestamp,
        end_timestamp,
    );

    let mut out = String::from("<span>");
    out += "<span class='stats-container'>";

    out += format!("<span>Category</span>").as_str();
    out += format!("<span>Actual</span>").as_str();
    out += format!("<span>Ideal</span>").as_str();
    out += format!("<span>Comp.</span>").as_str();
    out += format!("<span>Pred.</span>").as_str();
    out += format!("<span>Ratio</span>").as_str();

    let mut keys: Vec<&String> = tree.children.keys().into_iter().collect();
    keys.sort();

    let time_domain = keys
        .iter()
        .fold(0., |acc, x| tree.children[x.as_str()].value + acc);
    let ideal_domain = ideal_proportions.iter().fold(0., |acc, x| acc + x.1);

    let mut table_values = Vec::new();

    for key in keys.clone() {
        let mut ideal_value = 0.;

        let value = match ideal_proportions.get(key.as_str()) {
            Some(x) => x,
            None => continue,
        };

        ideal_value += 100. * value / ideal_domain;
        let actual_value = 100. * tree.children[key].value / time_domain;

        let color = match actual_value > ideal_value {
            false => "red",
            true => "green",
        };

        let weight = match current[0] == key.as_str() {
            false => "normal",
            true => "bold",
        };

        let percent_complete = 100. * actual_value / ideal_value;

        table_values.push((
            key,
            actual_value,
            ideal_value,
            color,
            percent_complete,
            weight,
            tree.children[key].value,
        ));
    }

    table_values.sort_by(|a, b| a.4.partial_cmp(&b.4).unwrap());

    for value in table_values {
        let key = value.0;
        let actual_value = value.1;
        let ideal_value = value.2;
        let color = value.3;
        let percent_complete = value.4;
        let weight = value.5;
        let duration = value.6;

        out += format!(
            "<span style='font-weight: {}; color: {}'> {}</span>",
            weight, color, key,
        )
        .as_str();
        out += format!(
            "<span style='font-weight: {}; color: {}'>{:.3}%</span>",
            weight, color, actual_value,
        )
        .as_str();
        out += format!(
            "<span style='font-weight: {}; color: {}'>{:.3}%</span>",
            weight, color, ideal_value
        )
        .as_str();
        out += format!(
            "<span style='font-weight: {}; color: {}'>{}</span>",
            weight,
            color,
            format_time(duration as i64)
        )
        .as_str();
        out += format!(
            "<span style='font-weight: {}; color: {}'>{}</span>",
            weight,
            color,
            format_time((ideal_value / 100. * 16. * 60. * 60.) as i64)
        )
        .as_str();
        out += format!(
            "<span style='font-weight: {}; color: {}'>{:.3}%</span>",
            weight, color, percent_complete
        )
        .as_str();
    }

    out += format!("<span></span>").as_str();
    out += format!("<span></span>").as_str();
    out += format!("<span></span>").as_str();
    out += format!("<span></span>").as_str();
    out += format!("<span></span>").as_str();
    out += format!("<span></span>").as_str();

    out += format!("{:.3} points", get_points(&tree, ideal_proportions)).as_str();

    out += "</span>";

    out + "</span>"
}

pub fn render_sankey(
    start_timestamp: i64,
    end_timestamp: i64,
    width: f64,
    height: f64,
    ideal_proportions: &HashMap<String, f64>,
) -> String {
    let (tree, current, _) = parse_file(
        "/home/sam/rofi_time_tracker/log",
        start_timestamp,
        end_timestamp,
    );

    let svg = render_tree(&tree, width, height, current, ideal_proportions);

    svg
}

pub fn render_band(start_timestamp: i64, end_timestamp: i64, width: f64, height: f64) -> String {
    let (_, _, band) = parse_file(
        "/home/sam/rofi_time_tracker/log",
        start_timestamp,
        end_timestamp,
    );

    let total= band.iter().fold(0, |acc, x| acc + x.0) as f64;

    let mut svg = format!("<svg width='100%' height='100%' xmlns='http://www.w3.org/2000/svg'>\n");
    let mut y = 0.;
    let x = 0.;
    for (duration, name) in band {
        let mut state = DefaultHasher::new();
        name.split(".").nth(0).unwrap().to_string()
            .hash(&mut state);
        let hue = state.finish() % 360;
        let color = format!("hsl({}, 30%, 50%)", hue);

        let width = width;
        let height = duration as f64/total*height;

        svg += format!(
            "<rect x='{}' y='{}' width='{}' height='{}' fill='{}' />\n",
            x, y, width, height, color
        )
        .as_str();

        y += height;
    }
    svg += "</svg>";

    svg
}
