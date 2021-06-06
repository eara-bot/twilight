use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr,
)]
#[repr(u8)]
pub enum PrivacyLevel {
    Public = 1,
    GuildOnly = 2,
}

#[cfg(test)]
mod tests {
    use super::PrivacyLevel;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&PrivacyLevel::Public, &[Token::U8(1)]);
        serde_test::assert_tokens(&PrivacyLevel::GuildOnly, &[Token::U8(2)]);
    }
}
