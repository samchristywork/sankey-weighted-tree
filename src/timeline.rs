use crate::parse_file;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

struct Row {
    key: String,
    delta: f64,
}

pub fn draw_timeline(filename: &str, width: f64, height: f64) -> String {
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let start_of_first_day = 1672552800;
    let mut current_day = start_of_first_day;

    let mut data: Vec<(Vec<Row>, i64)> = Vec::new();
    loop {
        let (tree, _) = parse_file(filename, current_day, current_day + 60 * 60 * 24);

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

        if current_day + 60 * 60 * 24 > current_time {
            break;
        }
    }

    let saturation = "30%";
    let lightness = "50%";

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
