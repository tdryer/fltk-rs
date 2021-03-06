use fltk::*;
use std::ops::{Deref, DerefMut};

struct MyButton {
    grp: group::Group,
    btn: button::Button,
}

impl MyButton {
    pub fn new(w: i32, h: i32, label: &str) -> MyButton {
        let mut grp = group::Group::new(0, 0, w, h, label);
        grp.set_frame(FrameType::RFlatBox);
        grp.set_color(Color::from_u32(0x01579b));
        grp.set_align(Align::Center);
        let mut btn = button::Button::new(grp.x() + 420, grp.y() + 35, 15, 15, "X");
        btn.set_frame(FrameType::OFlatFrame);
        btn.set_color(Color::from_u32(0xf49da9));
        btn.set_callback2(move |b| b.parent().unwrap().hide());
        grp.end();
        grp.handle2(|g, ev| match ev {
            Event::Push => {
                g.do_callback();
                true
            }
            _ => false,
        });
        MyButton { grp, btn }
    }
}

impl Deref for MyButton {
    type Target = group::Group;
    fn deref(&self) -> &Self::Target {
        &self.grp
    }
}
impl DerefMut for MyButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grp
    }
}

fn main() {
    let app = app::App::default();
    app::set_visible_focus(false);
    let mut win = window::Window::default().with_size(500, 400);
    win.make_resizable(true);
    win.set_color(Color::Black);
    let mut pack = group::Pack::default().size_of(&win);
    pack.set_spacing(10);

    for i in 0..3 {
        let label = format!("Button {}", i + 1);
        let mut but = MyButton::new(500, 100, &label);
        but.set_callback(move || println!("{}", label));
    }

    pack.end();
    win.end();
    win.show();
    app.run().unwrap();
}
