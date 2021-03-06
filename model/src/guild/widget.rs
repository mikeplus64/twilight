use crate::id::ChannelId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildWidget {
    pub channel_id: ChannelId,
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, GuildWidget};
    use serde_test::Token;

    #[test]
    fn test_guild_widget() {
        let prune = GuildWidget {
            channel_id: ChannelId(111_111_111_111_111_111),
            enabled: true,
        };

        serde_test::assert_tokens(
            &prune,
            &[
                Token::Struct {
                    name: "GuildWidget",
                    len: 2,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("111111111111111111"),
                Token::Str("enabled"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
