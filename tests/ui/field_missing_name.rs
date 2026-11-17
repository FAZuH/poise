use poise::view;
fn main() {
    let _ = view! { embed { field { value: "v", inline: true } } };
}
