use crate::component::Component;
use crate::point::Point;

pub struct ComponentBuilder {
    component: Component,
}

impl ComponentBuilder {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            component: Component::new(Point::new(x1, y1), Point::new(x2, y2)),
        }
    }

    pub fn color(mut self, color: &str) -> ComponentBuilder {
        self.component.set_color(color.to_string());
        self
    }

    pub fn left_text(mut self, text: &str) -> ComponentBuilder {
        self.component.set_text1(text.to_string());
        self
    }

    pub fn right_text(mut self, text: &str) -> ComponentBuilder {
        self.component.set_text2(text.to_string());
        self
    }

    pub fn height(mut self, height: f64) -> ComponentBuilder {
        self.component.set_bar_height(height);
        self
    }

    pub fn data(mut self, data: &str) -> ComponentBuilder {
        self.component.set_data(data.to_string());
        self
    }

    pub fn build(self) -> Component {
        self.component
    }
}
