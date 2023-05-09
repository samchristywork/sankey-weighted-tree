use std::collections::HashMap;
use crate::TreeNode;
use std::fs::File;
use std::io::Read;

fn parse_line(line: &str) -> (i64, String) {
    let mut words = line.split('\t');
    let epoch: i64 = words.next().unwrap().parse().unwrap();
    let tag = words.next().unwrap();

    (epoch, tag.to_string())
}

pub fn parse_file(filename: &str, begin: i64, end: i64) -> TreeNode {
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
