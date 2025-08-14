use std::env;

use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let input = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let inner_width = 66;

    let pattern_top = "´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:";
    let pattern_bot = ".•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•";

    fn make_line(pattern: &str, width: usize) -> String {
        let mut s = String::new();
        while s.chars().count() < width {
            s.push_str(pattern);
        }
        s.chars().take(width).collect()
    }

    let top_line = make_line(pattern_top, inner_width);
    let bot_line = make_line(pattern_bot, inner_width);

    let title = input.to_uppercase();
    let pad = inner_width.saturating_sub(title.chars().count());
    let left_pad = pad / 2;
    let right_pad = pad - left_pad;

    let output = format!(
        "    /*{top}*/\n    /*{lpad}{title}{rpad}*/\n    /*{bot}*/",
        top = top_line,
        lpad = " ".repeat(left_pad),
        title = title,
        rpad = " ".repeat(right_pad),
        bot = bot_line
    );

    println!("{}", output); // Print the header to console.

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    ctx.set_contents(output).unwrap(); // Copy the header to clipboard.
}
