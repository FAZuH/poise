use poise::view;

fn main() {
    let extra: Vec<poise::view_support::CreateComponent<'static>> = vec![];
    let _ = view! {
        components_v2 {
            text_display { "hi" }
        }
        { extra }
    };
}