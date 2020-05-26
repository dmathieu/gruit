use rocket_contrib::json::JsonValue;

#[get("/")]
pub fn index() -> Result<JsonValue, &'static str> {
    Ok(json!({"ok": true}))
}
