use std::collections::HashMap;

pub struct TreeNode {
    pub value: f64,
    pub children: HashMap<String, TreeNode>,
}

impl TreeNode {
    pub fn insert1(&mut self, activity: &str, time: f64) {
        self.value += time;
        if self.children.contains_key(activity) {
            let mut node = self.children.get_mut(activity).unwrap();
            node.value += time;
        } else {
            let node = TreeNode {
                value: time,
                children: HashMap::new(),
            };
            self.children.insert(activity.to_string(), node);
        }
    }
    pub fn insert2(&mut self, minor: &str, activity: &str, time: f64) {
        self.value += time;
        if self.children.contains_key(minor) {
            let node = self.children.get_mut(minor).unwrap();
            node.insert1(activity, time);
        } else {
            let mut node = TreeNode {
                value: 0.,
                children: HashMap::new(),
            };
            node.insert1(activity, time);
            self.children.insert(minor.to_string(), node);
        }
    }
    pub fn insert(&mut self, major: &str, minor: &str, activity: &str, time: f64) {
        self.value += time;
        if self.children.contains_key(major) {
            let node = self.children.get_mut(major).unwrap();
            node.insert2(minor, activity, time);
        } else {
            let mut node = TreeNode {
                value: 0.,
                children: HashMap::new(),
            };
            node.insert2(minor, activity, time);
            self.children.insert(major.to_string(), node);
        }
    }
}
