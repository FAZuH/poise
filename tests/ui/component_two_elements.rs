use poise::component;

fn main() {
    let _ = component! {
        text_display { "a" }
        text_display { "b" }
    };
}