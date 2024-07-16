//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{Recipient, True};

impl_payload! {
    /// Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the _can\_pin\_messages_ administrator right in the supergroup. Returns True on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub UnpinAllGeneralForumTopicMessages (UnpinAllGeneralForumTopicMessagesSetters) => True {
        required {
            /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
            pub chat_id: Recipient [into],
        }
    }
}
