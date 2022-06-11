use cursive::event::Key;
use cursive::traits::Identifiable;
// use cursive::views::StackView;
use cursive::views::{Checkbox, Dialog, EditView, ListView, TextView};
// use cursive::view::Margins;
// use terminal_size::{Width, Height, terminal_size};

const TEXT: &str = "Hello world!";
const TEXT2: &str = "
     O
    /-\\
     |
    / \\";

struct Options<'a> {
    message: &'a str,
    pokeball: bool,
}

fn main() {
    let mut app = cursive::default();
    input_step(&mut app);
    app.run();
}

fn input_step(app: &mut cursive::Cursive) {
    // let size = terminal_size();
    // let mut height: u16 = 0;
    // if let Some((Width(w), Height(h))) = size {
    //     height = h;
    // } else {}
    app.add_global_callback(Key::Esc, |s| s.quit());
    app.add_layer(
        Dialog::new()
        .title("        test         ")
        .content(
            ListView::new()
            .child("Message", EditView::new().with_name("message"))
            .child("Return!", Checkbox::new().with_name("return")), // .child(layer_sizes(), Checkbox::new().with_name("test"))
        )
        .button("start", |s| {
                let message = s
                .call_on_name("message", |t: &mut EditView| t.get_content())
                .unwrap();
                let returned = s
                .call_on_name("return", |t: &mut Checkbox| t.is_checked())
                .unwrap();
                let options = Options {
                    message: &message,
                    pokeball: returned,
                };
                result_step(s, &options)
            })
            // .padding(Margins::lrtb(30,30,10,10))
            // .set_padding_bottom(1),
        // .padding_bottom((height/5).into())
        
    );
        
}

fn result_step(app: &mut cursive::Cursive, options: &Options) {
    let text = if options.pokeball {
        format!("{}", TEXT)
    } else {
        format!(
            " {}
| {} |
 {}
 \\  /
  \\/
{}",
            "-".repeat(options.message.chars().count() + 2),
            options.message,
            "-".repeat(options.message.chars().count() + 2),
            TEXT2
        )
    };
    app.pop_layer();
    app.add_layer(
        Dialog::around(TextView::new(text))
            .title("World")
            .button("OK", |s| s.quit()),
    );
}
