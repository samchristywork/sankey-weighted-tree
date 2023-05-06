use crate::point::Point;

pub struct Component {
    color1: String,
    color2: String,
    a: Point,
    b: Point,
    bar_height: f64,
    text1: String,
    text2: String,
}

impl Component {
    pub fn new(a: Point, b: Point) -> Self {
        Self {
            color1: "red".to_string(),
            color2: "blue".to_string(),
            a,
            b,
            bar_height: 100.,
            text1: "".to_string(),
            text2: "".to_string(),
        }
    }

    pub fn set_text1(&mut self, text: String) {
        self.text1 = text;
    }

    pub fn set_text2(&mut self, text: String) {
        self.text2 = text;
    }

    pub fn set_color1(&mut self, color: String) {
        self.color1 = color;
    }

    pub fn set_color2(&mut self, color: String) {
        self.color2 = color;
    }

    pub fn set_bar_height(&mut self, height: f64) {
        self.bar_height = height;
    }

    pub fn draw(&self) -> String {
        let color1 = &self.color1;
        let color2 = &self.color2;
        let a = &self.a;
        let b = &self.b;
        let bar_height = &self.bar_height;
        let text1 = &self.text1;
        let text2 = &self.text2;

        let mx = (a.x + b.x) / 2.;
        let bar_width = 10.;

        let left_text = Point::new(a.x + bar_width, a.y + bar_height / 2.);
        let right_text = Point::new(b.x, b.y + bar_height / 2.);

        let top_left = Point::new(a.x + bar_width, a.y);
        let top_right = Point::new(b.x, b.y);
        let bottom_right = Point::new(b.x, b.y + bar_height);
        let bottom_left = Point::new(a.x, a.y + bar_height);

        let mut s = String::new();
        s += format!("<path d='").as_str();

        s += format!("M {} {}", top_left.x, top_left.y).as_str();
        s += format!(
            "C {} {}, {} {}, {} {}",
            mx, top_left.y, mx, top_right.y, top_right.x, top_right.y
        )
        .as_str();

        s += format!("L {} {}", bottom_right.x, bottom_right.y).as_str();
        s += format!(
            "C {} {}, {} {}, {} {}",
            mx, bottom_right.y, mx, bottom_left.y, bottom_left.x, bottom_left.y
        )
        .as_str();

        s += format!("' stroke='none' fill='#ccc' />\n").as_str();

        s += format!(
            "<rect x='{}' y='{}' width='{bar_width}' height='{bar_height}' fill='{color1}' />\n",
            a.x, a.y
        )
        .as_str();

        s += format!(
            "<rect x='{}' y='{}' width='{bar_width}' height='{bar_height}' fill='{color2}' />\n",
            b.x, b.y
        )
        .as_str();

        s += format!(
            "<text x='{}' y='{}' font-family='Verdana' font-size='12' fill='#222'>{text1}</text>\n",
            left_text.x, left_text.y
        )
        .as_str();
        s += format!("<text x='{}' y='{}' font-family='Verdana' font-size='12' fill='#222' text-anchor='end'>{text2}</text>\n", right_text.x, right_text.y).as_str();

        s
    }
}
