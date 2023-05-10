use crate::component_builder::ComponentBuilder;
use crate::parse::parse_file;
use crate::tree_node::TreeNode;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn render_tree(tree: &TreeNode, width: f64, height: f64, highlight: [String; 3]) -> String {
    let mut svg = format!("<svg width='90%' height='100%' xmlns='http://www.w3.org/2000/svg'>\n");

    let mut y = 10.;
    let factor = 1.4 * tree.value / height;
    let width = 0.985 * 0.6 * width / 3.;
    let mut innercount = 0.;
    let mut middlecount = 0.;
    let mut outercount = 0.;
    let step = 5.;

    let saturation = "50%";
    let lightness = "70%";

    let total_day_length = tree.value;

    let mut keys: Vec<&String> = tree.children.keys().into_iter().collect();
    keys.sort();
    for key in keys {
        let major = key;
        let x = 10.;

        let value = tree.children[key].value;

        let color = match major == highlight[0].as_str() {
            false => "#ccc",
            true => "#aaa",
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
            .data(
                format!(
                    "{label}: {:.2} minutes ({:.2}%)",
                    value / 60.,
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
                false => "#ccc",
                true => "#aaa",
            };

            let label = format!("{major}.{minor}");
            let mut state = DefaultHasher::new();
            label.hash(&mut state);
            let hue = state.finish() % 360;
            svg += ComponentBuilder::new(x, y + outercount, x + width - 10., y + middlecount)
                .height(value / factor)
                .color(format!("hsl({}, {saturation}, {lightness})", hue).as_str())
                .body_color(color)
                .right_text(label.as_str())
                .data(
                    format!(
                        "{label}: {:.2} minutes ({:.2}%)",
                        value / 60.,
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
                    false => "#ccc",
                    true => "#aaa",
                };

                let label = format!("{major}.{minor}.{activity}");
                let mut state = DefaultHasher::new();
                label.hash(&mut state);
                let hue = state.finish() % 360;
                svg += ComponentBuilder::new(x, y + middlecount, x + width - 10., y + innercount)
                    .height(value / factor)
                    .color(format!("hsl({}, {saturation}, {lightness})", hue).as_str())
                    .body_color(color)
                    .right_text(label.as_str())
                    .data(
                        format!(
                            "{label}: {:.2} minutes ({:.2}%)",
                            value / 60.,
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

    let mut state = DefaultHasher::new();
    "all".hash(&mut state);
    let hue = state.finish() % 360;
    y -= 10.;
    svg += format!("<rect x='0' y='10' width='10' height='{y}' fill='hsl({hue}, {saturation}, {lightness})' />\n").as_str();

    svg + "</svg>\n"
}

pub fn render_sankey(
    start_time: &String,
    end_time: &String,
    width: &String,
    height: &String,
) -> String {
    let (tree, current) = parse_file(
        "/home/sam/rofi_time_tracker/log",
        start_time.parse::<i64>().unwrap(),
        end_time.parse::<i64>().unwrap(),
    );

    let svg = render_tree(
        &tree,
        width.parse::<f64>().unwrap(),
        height.parse::<f64>().unwrap(),
        current,
    );

    svg
}
