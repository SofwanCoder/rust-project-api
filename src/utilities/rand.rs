use ulid::Ulid;
use uuid::Uuid;

pub fn generate_ulid() -> Ulid {
    return Ulid::new();
}
pub fn generate_uuid() -> Uuid {
    return generate_ulid().into();
}
