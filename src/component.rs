use crate::point::Point;

pub struct Component {
    color: String,
    a: Point,
    b: Point,
    bar_height: f64,
    text1: String,
    text2: String,
    data: String,
}

impl Component {
    pub fn new(a: Point, b: Point) -> Self {
        Self {
            color: "red".to_string(),
            a,
            b,
            bar_height: 100.,
            text1: "".to_string(),
            text2: "".to_string(),
            data: "".to_string(),
        }
    }

    pub fn set_text1(&mut self, text: String) {
        self.text1 = text;
    }

    pub fn set_text2(&mut self, text: String) {
        self.text2 = text;
    }

    pub fn set_color(&mut self, color: String) {
        self.color = color;
    }

    pub fn set_bar_height(&mut self, height: f64) {
        self.bar_height = height;
    }

    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }

    pub fn draw(&self) -> String {
        let color = &self.color;
        let a = &self.a;
        let b = &self.b;
        let bar_height = &self.bar_height;
        let text1 = &self.text1;
        let text2 = &self.text2;
        let text_padding = 5.;

        let mx = (a.x + b.x) / 2.;
        let bar_width = 10.;

        let left_text = Point::new(a.x + text_padding, a.y + bar_height / 2.);
        let right_text = Point::new(b.x - text_padding, b.y + bar_height / 2.);

        let top_left = Point::new(a.x, a.y);
        let top_right = Point::new(b.x, b.y);
        let bottom_right = Point::new(b.x, b.y + bar_height);
        let bottom_left = Point::new(a.x, a.y + bar_height);

        let mut s = String::new();
        s += format!(
            "<path class='hover-element' data-tooltip='{}' d='",
            &self.data
        )
        .as_str();

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
            "<rect x='{}' y='{}' width='{bar_width}' height='{bar_height}' fill='{color}' />\n",
            b.x, b.y
        )
        .as_str();

        let font_size = 12.;
        s += format!(
            "<text x='{}' y='{}' font-family='Verdana' font-size='{font_size}' fill='#222'>{text1}</text>\n",
            left_text.x, left_text.y+font_size/4.
        )
        .as_str();
        s += format!("<text x='{}' y='{}' font-family='Verdana' font-size='{font_size}' fill='#222' text-anchor='end'>{text2}</text>\n", right_text.x, right_text.y + font_size/4.).as_str();

        s
    }
}
