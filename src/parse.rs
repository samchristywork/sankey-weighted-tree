use crate::TreeNode;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn parse_line(line: &str) -> (u64, String) {
    let mut words = line.split('\t');
    let epoch: u64 = words.next().unwrap().parse().unwrap();
    let tag = words.next().unwrap();

    (epoch, tag.to_string())
}

pub fn parse_file(
    filename: &str,
    begin_timestamp: u64,
    end_timestamp: u64,
) -> (TreeNode, [String; 3], Vec<(u64, String)>) {
    let mut activities = Vec::new();

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents += format!("{}\tnow.now.now", end_timestamp).as_str();
    let mut lines = contents.lines();

    let mut last_line = parse_line(lines.next().unwrap());

    while let Some(line) = lines.next() {
        let contents = parse_line(line);
        let mut start_time = last_line.0;
        let mut end_time = contents.0;
        let activity = last_line.1.clone();

        if start_time < begin_timestamp {
            start_time = begin_timestamp;
        }

        if end_time > end_timestamp {
            end_time = end_timestamp;
        }

        let delta = end_time as i64 - start_time as i64;

        if delta > 0 && activity != "health.rest.sleep" {
            activities.push((delta, activity));
        }

        if contents.1 != "now.now.now" {
            last_line = contents;
        }
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

    (
        tree,
        [
            last_line.1.split('.').nth(0).unwrap().to_string(),
            last_line.1.split('.').nth(1).unwrap().to_string(),
            last_line.1.split('.').nth(2).unwrap().to_string(),
        ],
        activities,
    )
}
