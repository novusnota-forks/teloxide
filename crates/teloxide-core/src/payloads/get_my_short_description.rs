//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::BotShortDescription;

impl_payload! {
    /// Use this method to get the current bot short description for the given user language. Returns [`BotShortDescription`] on success.
    ///
    /// [`BotShortDescription`]: crate::types::BotShortDescription
    #[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Serialize)]
    pub GetMyShortDescription (GetMyShortDescriptionSetters) => BotShortDescription {
        optional {
            /// A two-letter ISO 639-1 language code or an empty string
            pub language_code: String [into],
        }
    }
}
