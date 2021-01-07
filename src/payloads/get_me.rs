// This file is auto generated by `cg` <https://github.com/teloxide/cg> (be02d84).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::User;

impl_payload! {
    /// A simple method for testing your bot's auth token. Requires no parameters. Returns basic information about the bot in form of a [`User`] object.
    ///
    /// [`User`]: crate::types::User
    #[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Serialize)]
    pub GetMe (GetMeSetters) => User {

    }
}
