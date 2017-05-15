#[get("/")]
fn index() -> &'static str {
    "Welcome!"
}