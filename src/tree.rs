use crate::component_builder::ComponentBuilder;
use crate::tree_node::TreeNode;
use crate::util::format_time;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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
    let component_width = width / 3. - 5.;
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
        svg += ComponentBuilder::new(x, y, x + component_width - 10., y + outercount)
            .height(value / factor)
            .color(format!("hsl({}, {saturation}, {lightness})", hue).as_str())
            .body_color(color)
            .right_text(label.as_str())
            .font_size(font_size)
            .data(
                format!(
                    "{label}: {} ({:.3}%)",
                    format_time(value as u64),
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
            let x = x + component_width;

            let value = tree.children[key].value;

            let color = match major == highlight[0].as_str() && minor == highlight[1].as_str() {
                false => "#444",
                true => "#222",
            };

            let label = format!("{major}.{minor}");
            let mut state = DefaultHasher::new();
            label.hash(&mut state);
            svg += ComponentBuilder::new(
                x,
                y + outercount,
                x + component_width - 10.,
                y + middlecount,
            )
            .height(value / factor)
            .color(format!("hsl({}, {saturation}, {lightness})", hue).as_str())
            .body_color(color)
            .right_text(label.as_str())
            .font_size(font_size)
            .data(
                format!(
                    "{label}: {} ({:.3}%)",
                    format_time(value as u64),
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
                let x = x + component_width;

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
                svg += ComponentBuilder::new(
                    x,
                    y + middlecount,
                    x + component_width - 10.,
                    y + innercount,
                )
                .height(value / factor)
                .color(format!("hsl({}, {saturation}, {lightness})", hue).as_str())
                .body_color(color)
                .right_text(label.as_str())
                .font_size(font_size)
                .data(
                    format!(
                        "{label}: {} ({:.3}%)",
                        format_time(value as u64),
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
