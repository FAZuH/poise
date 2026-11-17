use poise::view;
fn main() {
    let _ = view! {
        components_v2 {
            section {
                text_display { "a" }
                text_display { "b" }
                text_display { "c" }
                text_display { "d" }
                thumbnail { media: "https://example.test/a.png" }
            }
        }
    };
}
