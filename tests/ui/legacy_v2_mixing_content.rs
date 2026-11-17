use poise::view;
fn main() {
    let _ = view! {
        content: "hi"
        components_v2 { text_display { "hi" } }
    };
}
