use poise::view;

fn main() {
    let _ = view! {
        action_row {
            { vec![poise::view_support::CreateButton::new("b").label("B")] }
            select_menu { custom_id: "s" }
        }
    };
}