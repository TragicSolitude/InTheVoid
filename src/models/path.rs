use uuid::Uuid;

pub fn get_uuid() -> String {
    Uuid::new_v4()
        .simple()
        .to_string()
}