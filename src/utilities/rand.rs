use ulid::Ulid;
use uuid::Uuid;

pub fn generate_ulid() -> Ulid {
    return Ulid::new();
}
pub fn generate_uuid() -> Uuid {
    return Uuid::from_bytes(generate_ulid().to_bytes());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_ulid() {
        let ulid = generate_ulid();
        assert_eq!(ulid.to_string().len(), 26);
    }

    #[test]
    fn test_generate_uuid() {
        let uuid = generate_uuid();
        assert_eq!(uuid.to_string().len(), 36);
    }
}
