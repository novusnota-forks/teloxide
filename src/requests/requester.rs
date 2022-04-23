// We can't change Telegram API
#![allow(clippy::too_many_arguments)]

use url::Url;

use crate::{
    payloads::{GetMe, SendMessage, *},
    requests::Request,
    types::*,
};

/// Methods for building requests.
///
/// This trait is implemented by all bots & bot adaptors.
///
/// ## Examples
///
/// Calling TBA methods:
///
/// ```
/// # async {
/// # let chat_id = ChatId(-1);
/// use teloxide_core::{
///     prelude::*,
///     types::{ChatId, ParseMode},
/// };
///
/// // Bot implements `Requester`
/// let bot = Bot::new("TOKEN");
///
/// // Required parameters are supplied to the `Requester` methods:
/// bot.send_message(chat_id, "<b>Text</b>")
///     // Optional parameters can be supplied by calling setters
///     .parse_mode(ParseMode::Html)
///     // To send request to telegram you need to call `.send()` and await the resulting future
///     .send()
///     .await?;
/// # Ok::<_, teloxide_core::RequestError>(())
/// # };
/// ```
///
/// Using `Requester` in a generic context:
///
/// ```
/// use teloxide_core::{
///     prelude::*,
///     types::{ChatId, Message},
/// };
///
/// async fn send_hi<R>(bot: R, chat: ChatId) -> Message
/// where
///     R: Requester,
/// {
///     bot.send_message(chat, "hi").send().await.expect("error")
/// }
/// ```
#[cfg_attr(all(any(docsrs, dep_docsrs), feature = "nightly"), doc(notable_trait))]
pub trait Requester {
    /// Error type returned by all requests.
    type Err: std::error::Error + Send;

    // This block is auto generated by [`cg`] from [`schema`].
    //
    // **DO NOT EDIT THIS BLOCK**,
    //
    // Edit `cg` or `schema` instead.
    //
    // [cg]: https://github.com/teloxide/cg
    // [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema

    type GetUpdates: Request<Payload = GetUpdates, Err = Self::Err>;

    /// For Telegram documentation see [`GetUpdates`].
    fn get_updates(&self) -> Self::GetUpdates;

    type SetWebhook: Request<Payload = SetWebhook, Err = Self::Err>;

    /// For Telegram documentation see [`SetWebhook`].
    fn set_webhook(&self, url: Url) -> Self::SetWebhook;

    type DeleteWebhook: Request<Payload = DeleteWebhook, Err = Self::Err>;

    /// For Telegram documentation see [`DeleteWebhook`].
    fn delete_webhook(&self) -> Self::DeleteWebhook;

    type GetWebhookInfo: Request<Payload = GetWebhookInfo, Err = Self::Err>;

    /// For Telegram documentation see [`GetWebhookInfo`].
    fn get_webhook_info(&self) -> Self::GetWebhookInfo;

    type GetMe: Request<Payload = GetMe, Err = Self::Err>;

    /// For Telegram documentation see [`GetMe`].
    fn get_me(&self) -> Self::GetMe;

    type LogOut: Request<Payload = LogOut, Err = Self::Err>;

    /// For Telegram documentation see [`LogOut`].
    fn log_out(&self) -> Self::LogOut;

    type Close: Request<Payload = Close, Err = Self::Err>;

    /// For Telegram documentation see [`Close`].
    fn close(&self) -> Self::Close;

    type SendMessage: Request<Payload = SendMessage, Err = Self::Err>;

    /// For Telegram documentation see [`SendMessage`].
    fn send_message<C, T>(&self, chat_id: C, text: T) -> Self::SendMessage
    where
        C: Into<Recipient>,
        T: Into<String>;

    type ForwardMessage: Request<Payload = ForwardMessage, Err = Self::Err>;

    /// For Telegram documentation see [`ForwardMessage`].
    fn forward_message<C, F>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_id: i32,
    ) -> Self::ForwardMessage
    where
        C: Into<Recipient>,
        F: Into<Recipient>;

    type CopyMessage: Request<Payload = CopyMessage, Err = Self::Err>;

    /// For Telegram documentation see [`CopyMessage`].
    fn copy_message<C, F>(&self, chat_id: C, from_chat_id: F, message_id: i32) -> Self::CopyMessage
    where
        C: Into<Recipient>,
        F: Into<Recipient>;

    type SendPhoto: Request<Payload = SendPhoto, Err = Self::Err>;

    /// For Telegram documentation see [`SendPhoto`].
    fn send_photo<C>(&self, chat_id: C, photo: InputFile) -> Self::SendPhoto
    where
        C: Into<Recipient>;

    type SendAudio: Request<Payload = SendAudio, Err = Self::Err>;

    /// For Telegram documentation see [`SendAudio`].
    fn send_audio<C>(&self, chat_id: C, audio: InputFile) -> Self::SendAudio
    where
        C: Into<Recipient>;

    type SendDocument: Request<Payload = SendDocument, Err = Self::Err>;

    /// For Telegram documentation see [`SendDocument`].
    fn send_document<C>(&self, chat_id: C, document: InputFile) -> Self::SendDocument
    where
        C: Into<Recipient>;

    type SendVideo: Request<Payload = SendVideo, Err = Self::Err>;

    /// For Telegram documentation see [`SendVideo`].
    fn send_video<C>(&self, chat_id: C, video: InputFile) -> Self::SendVideo
    where
        C: Into<Recipient>;

    type SendAnimation: Request<Payload = SendAnimation, Err = Self::Err>;

    /// For Telegram documentation see [`SendAnimation`].
    fn send_animation<C>(&self, chat_id: C, animation: InputFile) -> Self::SendAnimation
    where
        C: Into<Recipient>;

    type SendVoice: Request<Payload = SendVoice, Err = Self::Err>;

    /// For Telegram documentation see [`SendVoice`].
    fn send_voice<C>(&self, chat_id: C, voice: InputFile) -> Self::SendVoice
    where
        C: Into<Recipient>;

    type SendVideoNote: Request<Payload = SendVideoNote, Err = Self::Err>;

    /// For Telegram documentation see [`SendVideoNote`].
    fn send_video_note<C>(&self, chat_id: C, video_note: InputFile) -> Self::SendVideoNote
    where
        C: Into<Recipient>;

    type SendMediaGroup: Request<Payload = SendMediaGroup, Err = Self::Err>;

    /// For Telegram documentation see [`SendMediaGroup`].
    fn send_media_group<C, M>(&self, chat_id: C, media: M) -> Self::SendMediaGroup
    where
        C: Into<Recipient>,
        M: IntoIterator<Item = InputMedia>;

    type SendLocation: Request<Payload = SendLocation, Err = Self::Err>;

    /// For Telegram documentation see [`SendLocation`].
    fn send_location<C>(&self, chat_id: C, latitude: f64, longitude: f64) -> Self::SendLocation
    where
        C: Into<Recipient>;

    type EditMessageLiveLocation: Request<Payload = EditMessageLiveLocation, Err = Self::Err>;

    /// For Telegram documentation see [`EditMessageLiveLocation`].
    fn edit_message_live_location<C>(
        &self,
        chat_id: C,
        message_id: i32,
        latitude: f64,
        longitude: f64,
    ) -> Self::EditMessageLiveLocation
    where
        C: Into<Recipient>;

    type EditMessageLiveLocationInline: Request<
        Payload = EditMessageLiveLocationInline,
        Err = Self::Err,
    >;

    /// For Telegram documentation see [`EditMessageLiveLocationInline`].
    fn edit_message_live_location_inline<I>(
        &self,
        inline_message_id: I,
        latitude: f64,
        longitude: f64,
    ) -> Self::EditMessageLiveLocationInline
    where
        I: Into<String>;

    type StopMessageLiveLocation: Request<Payload = StopMessageLiveLocation, Err = Self::Err>;

    /// For Telegram documentation see [`StopMessageLiveLocation`].
    fn stop_message_live_location<C>(
        &self,
        chat_id: C,
        message_id: i32,
        latitude: f64,
        longitude: f64,
    ) -> Self::StopMessageLiveLocation
    where
        C: Into<Recipient>;

    type StopMessageLiveLocationInline: Request<
        Payload = StopMessageLiveLocationInline,
        Err = Self::Err,
    >;

    /// For Telegram documentation see [`StopMessageLiveLocationInline`].
    fn stop_message_live_location_inline<I>(
        &self,
        inline_message_id: I,
        latitude: f64,
        longitude: f64,
    ) -> Self::StopMessageLiveLocationInline
    where
        I: Into<String>;

    type SendVenue: Request<Payload = SendVenue, Err = Self::Err>;

    /// For Telegram documentation see [`SendVenue`].
    fn send_venue<C, T, A>(
        &self,
        chat_id: C,
        latitude: f64,
        longitude: f64,
        title: T,
        address: A,
    ) -> Self::SendVenue
    where
        C: Into<Recipient>,
        T: Into<String>,
        A: Into<String>;

    type SendContact: Request<Payload = SendContact, Err = Self::Err>;

    /// For Telegram documentation see [`SendContact`].
    fn send_contact<C, P, F>(
        &self,
        chat_id: C,
        phone_number: P,
        first_name: F,
    ) -> Self::SendContact
    where
        C: Into<Recipient>,
        P: Into<String>,
        F: Into<String>;

    type SendPoll: Request<Payload = SendPoll, Err = Self::Err>;

    /// For Telegram documentation see [`SendPoll`].
    fn send_poll<C, Q, O>(&self, chat_id: C, question: Q, options: O) -> Self::SendPoll
    where
        C: Into<Recipient>,
        Q: Into<String>,
        O: IntoIterator<Item = String>;

    type SendDice: Request<Payload = SendDice, Err = Self::Err>;

    /// For Telegram documentation see [`SendDice`].
    fn send_dice<C>(&self, chat_id: C) -> Self::SendDice
    where
        C: Into<Recipient>;

    type SendChatAction: Request<Payload = SendChatAction, Err = Self::Err>;

    /// For Telegram documentation see [`SendChatAction`].
    fn send_chat_action<C>(&self, chat_id: C, action: ChatAction) -> Self::SendChatAction
    where
        C: Into<Recipient>;

    type GetUserProfilePhotos: Request<Payload = GetUserProfilePhotos, Err = Self::Err>;

    /// For Telegram documentation see [`GetUserProfilePhotos`].
    fn get_user_profile_photos(&self, user_id: UserId) -> Self::GetUserProfilePhotos;

    type GetFile: Request<Payload = GetFile, Err = Self::Err>;

    /// For Telegram documentation see [`GetFile`].
    fn get_file<F>(&self, file_id: F) -> Self::GetFile
    where
        F: Into<String>;

    type BanChatMember: Request<Payload = BanChatMember, Err = Self::Err>;

    /// For Telegram documentation see [`BanChatMember`].
    fn ban_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::BanChatMember
    where
        C: Into<Recipient>;

    type KickChatMember: Request<Payload = KickChatMember, Err = Self::Err>;

    /// For Telegram documentation see [`KickChatMember`].
    fn kick_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::KickChatMember
    where
        C: Into<Recipient>;

    type UnbanChatMember: Request<Payload = UnbanChatMember, Err = Self::Err>;

    /// For Telegram documentation see [`UnbanChatMember`].
    fn unban_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::UnbanChatMember
    where
        C: Into<Recipient>;

    type RestrictChatMember: Request<Payload = RestrictChatMember, Err = Self::Err>;

    /// For Telegram documentation see [`RestrictChatMember`].
    fn restrict_chat_member<C>(
        &self,
        chat_id: C,
        user_id: UserId,
        permissions: ChatPermissions,
    ) -> Self::RestrictChatMember
    where
        C: Into<Recipient>;

    type PromoteChatMember: Request<Payload = PromoteChatMember, Err = Self::Err>;

    /// For Telegram documentation see [`PromoteChatMember`].
    fn promote_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::PromoteChatMember
    where
        C: Into<Recipient>;

    type SetChatAdministratorCustomTitle: Request<
        Payload = SetChatAdministratorCustomTitle,
        Err = Self::Err,
    >;

    /// For Telegram documentation see [`SetChatAdministratorCustomTitle`].
    fn set_chat_administrator_custom_title<Ch, Cu>(
        &self,
        chat_id: Ch,
        user_id: UserId,
        custom_title: Cu,
    ) -> Self::SetChatAdministratorCustomTitle
    where
        Ch: Into<Recipient>,
        Cu: Into<String>;

    type BanChatSenderChat: Request<Payload = BanChatSenderChat, Err = Self::Err>;

    /// For Telegram documentation see [`BanChatSenderChat`].
    fn ban_chat_sender_chat<C, S>(&self, chat_id: C, sender_chat_id: S) -> Self::BanChatSenderChat
    where
        C: Into<Recipient>,
        S: Into<ChatId>;

    type UnbanChatSenderChat: Request<Payload = UnbanChatSenderChat, Err = Self::Err>;

    /// For Telegram documentation see [`UnbanChatSenderChat`].
    fn unban_chat_sender_chat<C, S>(
        &self,
        chat_id: C,
        sender_chat_id: S,
    ) -> Self::UnbanChatSenderChat
    where
        C: Into<Recipient>,
        S: Into<ChatId>;

    type SetChatPermissions: Request<Payload = SetChatPermissions, Err = Self::Err>;

    /// For Telegram documentation see [`SetChatPermissions`].
    fn set_chat_permissions<C>(
        &self,
        chat_id: C,
        permissions: ChatPermissions,
    ) -> Self::SetChatPermissions
    where
        C: Into<Recipient>;

    type ExportChatInviteLink: Request<Payload = ExportChatInviteLink, Err = Self::Err>;

    /// For Telegram documentation see [`ExportChatInviteLink`].
    fn export_chat_invite_link<C>(&self, chat_id: C) -> Self::ExportChatInviteLink
    where
        C: Into<Recipient>;

    type CreateChatInviteLink: Request<Payload = CreateChatInviteLink, Err = Self::Err>;

    /// For Telegram documentation see [`CreateChatInviteLink`].
    fn create_chat_invite_link<C>(&self, chat_id: C) -> Self::CreateChatInviteLink
    where
        C: Into<Recipient>;

    type EditChatInviteLink: Request<Payload = EditChatInviteLink, Err = Self::Err>;

    /// For Telegram documentation see [`EditChatInviteLink`].
    fn edit_chat_invite_link<C, I>(&self, chat_id: C, invite_link: I) -> Self::EditChatInviteLink
    where
        C: Into<Recipient>,
        I: Into<String>;

    type RevokeChatInviteLink: Request<Payload = RevokeChatInviteLink, Err = Self::Err>;

    /// For Telegram documentation see [`RevokeChatInviteLink`].
    fn revoke_chat_invite_link<C, I>(
        &self,
        chat_id: C,
        invite_link: I,
    ) -> Self::RevokeChatInviteLink
    where
        C: Into<Recipient>,
        I: Into<String>;

    type ApproveChatJoinRequest: Request<Payload = ApproveChatJoinRequest, Err = Self::Err>;

    /// For Telegram documentation see [`ApproveChatJoinRequest`].
    fn approve_chat_join_request<C>(
        &self,
        chat_id: C,
        user_id: UserId,
    ) -> Self::ApproveChatJoinRequest
    where
        C: Into<Recipient>;

    type DeclineChatJoinRequest: Request<Payload = DeclineChatJoinRequest, Err = Self::Err>;

    /// For Telegram documentation see [`DeclineChatJoinRequest`].
    fn decline_chat_join_request<C>(
        &self,
        chat_id: C,
        user_id: UserId,
    ) -> Self::DeclineChatJoinRequest
    where
        C: Into<Recipient>;

    type SetChatPhoto: Request<Payload = SetChatPhoto, Err = Self::Err>;

    /// For Telegram documentation see [`SetChatPhoto`].
    fn set_chat_photo<C>(&self, chat_id: C, photo: InputFile) -> Self::SetChatPhoto
    where
        C: Into<Recipient>;

    type DeleteChatPhoto: Request<Payload = DeleteChatPhoto, Err = Self::Err>;

    /// For Telegram documentation see [`DeleteChatPhoto`].
    fn delete_chat_photo<C>(&self, chat_id: C) -> Self::DeleteChatPhoto
    where
        C: Into<Recipient>;

    type SetChatTitle: Request<Payload = SetChatTitle, Err = Self::Err>;

    /// For Telegram documentation see [`SetChatTitle`].
    fn set_chat_title<C, T>(&self, chat_id: C, title: T) -> Self::SetChatTitle
    where
        C: Into<Recipient>,
        T: Into<String>;

    type SetChatDescription: Request<Payload = SetChatDescription, Err = Self::Err>;

    /// For Telegram documentation see [`SetChatDescription`].
    fn set_chat_description<C>(&self, chat_id: C) -> Self::SetChatDescription
    where
        C: Into<Recipient>;

    type PinChatMessage: Request<Payload = PinChatMessage, Err = Self::Err>;

    /// For Telegram documentation see [`PinChatMessage`].
    fn pin_chat_message<C>(&self, chat_id: C, message_id: i32) -> Self::PinChatMessage
    where
        C: Into<Recipient>;

    type UnpinChatMessage: Request<Payload = UnpinChatMessage, Err = Self::Err>;

    /// For Telegram documentation see [`UnpinChatMessage`].
    fn unpin_chat_message<C>(&self, chat_id: C) -> Self::UnpinChatMessage
    where
        C: Into<Recipient>;

    type UnpinAllChatMessages: Request<Payload = UnpinAllChatMessages, Err = Self::Err>;

    /// For Telegram documentation see [`UnpinAllChatMessages`].
    fn unpin_all_chat_messages<C>(&self, chat_id: C) -> Self::UnpinAllChatMessages
    where
        C: Into<Recipient>;

    type LeaveChat: Request<Payload = LeaveChat, Err = Self::Err>;

    /// For Telegram documentation see [`LeaveChat`].
    fn leave_chat<C>(&self, chat_id: C) -> Self::LeaveChat
    where
        C: Into<Recipient>;

    type GetChat: Request<Payload = GetChat, Err = Self::Err>;

    /// For Telegram documentation see [`GetChat`].
    fn get_chat<C>(&self, chat_id: C) -> Self::GetChat
    where
        C: Into<Recipient>;

    type GetChatAdministrators: Request<Payload = GetChatAdministrators, Err = Self::Err>;

    /// For Telegram documentation see [`GetChatAdministrators`].
    fn get_chat_administrators<C>(&self, chat_id: C) -> Self::GetChatAdministrators
    where
        C: Into<Recipient>;

    type GetChatMemberCount: Request<Payload = GetChatMemberCount, Err = Self::Err>;

    /// For Telegram documentation see [`GetChatMemberCount`].
    fn get_chat_member_count<C>(&self, chat_id: C) -> Self::GetChatMemberCount
    where
        C: Into<Recipient>;

    type GetChatMembersCount: Request<Payload = GetChatMembersCount, Err = Self::Err>;

    /// For Telegram documentation see [`GetChatMembersCount`].
    fn get_chat_members_count<C>(&self, chat_id: C) -> Self::GetChatMembersCount
    where
        C: Into<Recipient>;

    type GetChatMember: Request<Payload = GetChatMember, Err = Self::Err>;

    /// For Telegram documentation see [`GetChatMember`].
    fn get_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::GetChatMember
    where
        C: Into<Recipient>;

    type SetChatStickerSet: Request<Payload = SetChatStickerSet, Err = Self::Err>;

    /// For Telegram documentation see [`SetChatStickerSet`].
    fn set_chat_sticker_set<C, S>(
        &self,
        chat_id: C,
        sticker_set_name: S,
    ) -> Self::SetChatStickerSet
    where
        C: Into<Recipient>,
        S: Into<String>;

    type DeleteChatStickerSet: Request<Payload = DeleteChatStickerSet, Err = Self::Err>;

    /// For Telegram documentation see [`DeleteChatStickerSet`].
    fn delete_chat_sticker_set<C>(&self, chat_id: C) -> Self::DeleteChatStickerSet
    where
        C: Into<Recipient>;

    type AnswerCallbackQuery: Request<Payload = AnswerCallbackQuery, Err = Self::Err>;

    /// For Telegram documentation see [`AnswerCallbackQuery`].
    fn answer_callback_query<C>(&self, callback_query_id: C) -> Self::AnswerCallbackQuery
    where
        C: Into<String>;

    type SetMyCommands: Request<Payload = SetMyCommands, Err = Self::Err>;

    /// For Telegram documentation see [`SetMyCommands`].
    fn set_my_commands<C>(&self, commands: C) -> Self::SetMyCommands
    where
        C: IntoIterator<Item = BotCommand>;

    type GetMyCommands: Request<Payload = GetMyCommands, Err = Self::Err>;

    /// For Telegram documentation see [`GetMyCommands`].
    fn get_my_commands(&self) -> Self::GetMyCommands;

    type SetChatMenuButton: Request<Payload = SetChatMenuButton, Err = Self::Err>;

    /// For Telegram documentation see [`SetChatMenuButton`].
    fn set_chat_menu_button(&self) -> Self::SetChatMenuButton;

    type GetChatMenuButton: Request<Payload = GetChatMenuButton, Err = Self::Err>;

    /// For Telegram documentation see [`GetChatMenuButton`].
    fn get_chat_menu_button(&self) -> Self::GetChatMenuButton;

    type SetMyDefaultAdministratorRights: Request<
        Payload = SetMyDefaultAdministratorRights,
        Err = Self::Err,
    >;

    /// For Telegram documentation see [`SetMyDefaultAdministratorRights`].
    fn set_my_default_administrator_rights(&self) -> Self::SetMyDefaultAdministratorRights;

    type GetMyDefaultAdministratorRights: Request<
        Payload = GetMyDefaultAdministratorRights,
        Err = Self::Err,
    >;

    /// For Telegram documentation see [`GetMyDefaultAdministratorRights`].
    fn get_my_default_administrator_rights(&self) -> Self::GetMyDefaultAdministratorRights;

    type DeleteMyCommands: Request<Payload = DeleteMyCommands, Err = Self::Err>;

    /// For Telegram documentation see [`DeleteMyCommands`].
    fn delete_my_commands(&self) -> Self::DeleteMyCommands;

    type AnswerInlineQuery: Request<Payload = AnswerInlineQuery, Err = Self::Err>;

    /// For Telegram documentation see [`AnswerInlineQuery`].
    fn answer_inline_query<I, R>(&self, inline_query_id: I, results: R) -> Self::AnswerInlineQuery
    where
        I: Into<String>,
        R: IntoIterator<Item = InlineQueryResult>;

    type AnswerWebAppQuery: Request<Payload = AnswerWebAppQuery, Err = Self::Err>;

    /// For Telegram documentation see [`AnswerWebAppQuery`].
    fn answer_web_app_query<W>(
        &self,
        web_app_query_id: W,
        result: InlineQueryResult,
    ) -> Self::AnswerWebAppQuery
    where
        W: Into<String>;

    type EditMessageText: Request<Payload = EditMessageText, Err = Self::Err>;

    /// For Telegram documentation see [`EditMessageText`].
    fn edit_message_text<C, T>(
        &self,
        chat_id: C,
        message_id: i32,
        text: T,
    ) -> Self::EditMessageText
    where
        C: Into<Recipient>,
        T: Into<String>;

    type EditMessageTextInline: Request<Payload = EditMessageTextInline, Err = Self::Err>;

    /// For Telegram documentation see [`EditMessageTextInline`].
    fn edit_message_text_inline<I, T>(
        &self,
        inline_message_id: I,
        text: T,
    ) -> Self::EditMessageTextInline
    where
        I: Into<String>,
        T: Into<String>;

    type EditMessageCaption: Request<Payload = EditMessageCaption, Err = Self::Err>;

    /// For Telegram documentation see [`EditMessageCaption`].
    fn edit_message_caption<C>(&self, chat_id: C, message_id: i32) -> Self::EditMessageCaption
    where
        C: Into<Recipient>;

    type EditMessageCaptionInline: Request<Payload = EditMessageCaptionInline, Err = Self::Err>;

    /// For Telegram documentation see [`EditMessageCaptionInline`].
    fn edit_message_caption_inline<I>(
        &self,
        inline_message_id: I,
    ) -> Self::EditMessageCaptionInline
    where
        I: Into<String>;

    type EditMessageMedia: Request<Payload = EditMessageMedia, Err = Self::Err>;

    /// For Telegram documentation see [`EditMessageMedia`].
    fn edit_message_media<C>(
        &self,
        chat_id: C,
        message_id: i32,
        media: InputMedia,
    ) -> Self::EditMessageMedia
    where
        C: Into<Recipient>;

    type EditMessageMediaInline: Request<Payload = EditMessageMediaInline, Err = Self::Err>;

    /// For Telegram documentation see [`EditMessageMediaInline`].
    fn edit_message_media_inline<I>(
        &self,
        inline_message_id: I,
        media: InputMedia,
    ) -> Self::EditMessageMediaInline
    where
        I: Into<String>;

    type EditMessageReplyMarkup: Request<Payload = EditMessageReplyMarkup, Err = Self::Err>;

    /// For Telegram documentation see [`EditMessageReplyMarkup`].
    fn edit_message_reply_markup<C>(
        &self,
        chat_id: C,
        message_id: i32,
    ) -> Self::EditMessageReplyMarkup
    where
        C: Into<Recipient>;

    type EditMessageReplyMarkupInline: Request<
        Payload = EditMessageReplyMarkupInline,
        Err = Self::Err,
    >;

    /// For Telegram documentation see [`EditMessageReplyMarkupInline`].
    fn edit_message_reply_markup_inline<I>(
        &self,
        inline_message_id: I,
    ) -> Self::EditMessageReplyMarkupInline
    where
        I: Into<String>;

    type StopPoll: Request<Payload = StopPoll, Err = Self::Err>;

    /// For Telegram documentation see [`StopPoll`].
    fn stop_poll<C>(&self, chat_id: C, message_id: i32) -> Self::StopPoll
    where
        C: Into<Recipient>;

    type DeleteMessage: Request<Payload = DeleteMessage, Err = Self::Err>;

    /// For Telegram documentation see [`DeleteMessage`].
    fn delete_message<C>(&self, chat_id: C, message_id: i32) -> Self::DeleteMessage
    where
        C: Into<Recipient>;

    type SendSticker: Request<Payload = SendSticker, Err = Self::Err>;

    /// For Telegram documentation see [`SendSticker`].
    fn send_sticker<C>(&self, chat_id: C, sticker: InputFile) -> Self::SendSticker
    where
        C: Into<Recipient>;

    type GetStickerSet: Request<Payload = GetStickerSet, Err = Self::Err>;

    /// For Telegram documentation see [`GetStickerSet`].
    fn get_sticker_set<N>(&self, name: N) -> Self::GetStickerSet
    where
        N: Into<String>;

    type UploadStickerFile: Request<Payload = UploadStickerFile, Err = Self::Err>;

    /// For Telegram documentation see [`UploadStickerFile`].
    fn upload_sticker_file(
        &self,
        user_id: UserId,
        png_sticker: InputFile,
    ) -> Self::UploadStickerFile;

    type CreateNewStickerSet: Request<Payload = CreateNewStickerSet, Err = Self::Err>;

    /// For Telegram documentation see [`CreateNewStickerSet`].
    fn create_new_sticker_set<N, T, E>(
        &self,
        user_id: UserId,
        name: N,
        title: T,
        sticker: InputSticker,
        emojis: E,
    ) -> Self::CreateNewStickerSet
    where
        N: Into<String>,
        T: Into<String>,
        E: Into<String>;

    type AddStickerToSet: Request<Payload = AddStickerToSet, Err = Self::Err>;

    /// For Telegram documentation see [`AddStickerToSet`].
    fn add_sticker_to_set<N, E>(
        &self,
        user_id: UserId,
        name: N,
        sticker: InputSticker,
        emojis: E,
    ) -> Self::AddStickerToSet
    where
        N: Into<String>,
        E: Into<String>;

    type SetStickerPositionInSet: Request<Payload = SetStickerPositionInSet, Err = Self::Err>;

    /// For Telegram documentation see [`SetStickerPositionInSet`].
    fn set_sticker_position_in_set<S>(
        &self,
        sticker: S,
        position: u32,
    ) -> Self::SetStickerPositionInSet
    where
        S: Into<String>;

    type DeleteStickerFromSet: Request<Payload = DeleteStickerFromSet, Err = Self::Err>;

    /// For Telegram documentation see [`DeleteStickerFromSet`].
    fn delete_sticker_from_set<S>(&self, sticker: S) -> Self::DeleteStickerFromSet
    where
        S: Into<String>;

    type SetStickerSetThumb: Request<Payload = SetStickerSetThumb, Err = Self::Err>;

    /// For Telegram documentation see [`SetStickerSetThumb`].
    fn set_sticker_set_thumb<N>(&self, name: N, user_id: UserId) -> Self::SetStickerSetThumb
    where
        N: Into<String>;

    type SendInvoice: Request<Payload = SendInvoice, Err = Self::Err>;

    /// For Telegram documentation see [`SendInvoice`].
    fn send_invoice<Ch, T, D, Pa, P, C, Pri>(
        &self,
        chat_id: Ch,
        title: T,
        description: D,
        payload: Pa,
        provider_token: P,
        currency: C,
        prices: Pri,
    ) -> Self::SendInvoice
    where
        Ch: Into<Recipient>,
        T: Into<String>,
        D: Into<String>,
        Pa: Into<String>,
        P: Into<String>,
        C: Into<String>,
        Pri: IntoIterator<Item = LabeledPrice>;

    type AnswerShippingQuery: Request<Payload = AnswerShippingQuery, Err = Self::Err>;

    /// For Telegram documentation see [`AnswerShippingQuery`].
    fn answer_shipping_query<S>(&self, shipping_query_id: S, ok: bool) -> Self::AnswerShippingQuery
    where
        S: Into<String>;

    type AnswerPreCheckoutQuery: Request<Payload = AnswerPreCheckoutQuery, Err = Self::Err>;

    /// For Telegram documentation see [`AnswerPreCheckoutQuery`].
    fn answer_pre_checkout_query<P>(
        &self,
        pre_checkout_query_id: P,
        ok: bool,
    ) -> Self::AnswerPreCheckoutQuery
    where
        P: Into<String>;

    type SetPassportDataErrors: Request<Payload = SetPassportDataErrors, Err = Self::Err>;

    /// For Telegram documentation see [`SetPassportDataErrors`].
    fn set_passport_data_errors<E>(
        &self,
        user_id: UserId,
        errors: E,
    ) -> Self::SetPassportDataErrors
    where
        E: IntoIterator<Item = PassportElementError>;

    type SendGame: Request<Payload = SendGame, Err = Self::Err>;

    /// For Telegram documentation see [`SendGame`].
    fn send_game<G>(&self, chat_id: u32, game_short_name: G) -> Self::SendGame
    where
        G: Into<String>;

    type SetGameScore: Request<Payload = SetGameScore, Err = Self::Err>;

    /// For Telegram documentation see [`SetGameScore`].
    fn set_game_score(
        &self,
        user_id: UserId,
        score: u64,
        chat_id: u32,
        message_id: i64,
    ) -> Self::SetGameScore;

    type SetGameScoreInline: Request<Payload = SetGameScoreInline, Err = Self::Err>;

    /// For Telegram documentation see [`SetGameScoreInline`].
    fn set_game_score_inline<I>(
        &self,
        user_id: UserId,
        score: u64,
        inline_message_id: I,
    ) -> Self::SetGameScoreInline
    where
        I: Into<String>;

    type GetGameHighScores: Request<Payload = GetGameHighScores, Err = Self::Err>;

    /// For Telegram documentation see [`GetGameHighScores`].
    fn get_game_high_scores<T>(&self, user_id: UserId, target: T) -> Self::GetGameHighScores
    where
        T: Into<TargetMessage>;
}

macro_rules! fty {
    ($T:ident) => {
        B::$T
    };
}

macro_rules! fwd_deref {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        core::ops::Deref::deref($this).$m($($arg),*)
    };
}

macro_rules! forward_all {
    () => {
        requester_forward! {
            get_me,
            log_out,
            close,
            get_updates,
            set_webhook,
            delete_webhook,
            get_webhook_info,
            forward_message,
            copy_message,
            send_message,
            send_photo,
            send_audio,
            send_document,
            send_video,
            send_animation,
            send_voice,
            send_video_note,
            send_media_group,
            send_location,
            edit_message_live_location,
            edit_message_live_location_inline,
            stop_message_live_location,
            stop_message_live_location_inline,
            send_venue,
            send_contact,
            send_poll,
            send_dice,
            send_chat_action,
            get_user_profile_photos,
            get_file,
            kick_chat_member,
            ban_chat_member,
            unban_chat_member,
            restrict_chat_member,
            promote_chat_member,
            set_chat_administrator_custom_title,
            ban_chat_sender_chat,
            unban_chat_sender_chat,
            set_chat_permissions,
            export_chat_invite_link,
            create_chat_invite_link,
            edit_chat_invite_link,
            revoke_chat_invite_link,
            set_chat_photo,
            delete_chat_photo,
            set_chat_title,
            set_chat_description,
            pin_chat_message,
            unpin_chat_message,
            unpin_all_chat_messages,
            leave_chat,
            get_chat,
            get_chat_administrators,
            get_chat_members_count,
            get_chat_member_count,
            get_chat_member,
            set_chat_sticker_set,
            delete_chat_sticker_set,
            answer_callback_query,
            set_my_commands,
            get_my_commands,
            set_chat_menu_button,
            get_chat_menu_button,
            set_my_default_administrator_rights,
            get_my_default_administrator_rights,
            delete_my_commands,
            answer_inline_query,
            answer_web_app_query,
            edit_message_text,
            edit_message_text_inline,
            edit_message_caption,
            edit_message_caption_inline,
            edit_message_media,
            edit_message_media_inline,
            edit_message_reply_markup,
            edit_message_reply_markup_inline,
            stop_poll,
            delete_message,
            send_sticker,
            get_sticker_set,
            upload_sticker_file,
            create_new_sticker_set,
            add_sticker_to_set,
            set_sticker_position_in_set,
            delete_sticker_from_set,
            set_sticker_set_thumb,
            send_invoice,
            answer_shipping_query,
            answer_pre_checkout_query,
            set_passport_data_errors,
            send_game,
            set_game_score,
            set_game_score_inline,
            get_game_high_scores,
            approve_chat_join_request,
            decline_chat_join_request
            => fwd_deref, fty
        }
    };
}

impl<B> Requester for &B
where
    B: Requester,
{
    type Err = B::Err;

    forward_all! {}
}

impl<B> Requester for &mut B
where
    B: Requester,
{
    type Err = B::Err;

    forward_all! {}
}

impl<B> Requester for Box<B>
where
    B: Requester,
{
    type Err = B::Err;

    forward_all! {}
}

impl<B> Requester for std::sync::Arc<B>
where
    B: Requester,
{
    type Err = B::Err;

    forward_all! {}
}

impl<B> Requester for std::rc::Rc<B>
where
    B: Requester,
{
    type Err = B::Err;

    forward_all! {}
}

macro_rules! fty_either {
    ($T:ident) => {
        either::Either<LR::$T, RR::$T>
    };
}

macro_rules! fwd_either {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        match ($this) {
            either::Either::Left(l) => either::Either::Left(l.$m($($arg),*)),
            either::Either::Right(r) => either::Either::Right(r.$m($($arg),*)),
        }
    };
}

impl<LR, RR> Requester for either::Either<LR, RR>
where
    LR: Requester,
    RR: Requester<Err = LR::Err>,
{
    type Err = LR::Err;

    requester_forward! {
        get_me, log_out, close, get_updates, set_webhook, delete_webhook, get_webhook_info,
        forward_message, copy_message, send_message, send_photo, send_audio, send_document,
        send_video, send_animation, send_voice, send_video_note, send_media_group, send_location,
        edit_message_live_location, edit_message_live_location_inline,
        stop_message_live_location, stop_message_live_location_inline, send_venue,
        send_contact, send_poll, send_dice, send_chat_action, get_user_profile_photos,
        get_file, kick_chat_member, ban_chat_member, unban_chat_member, restrict_chat_member,
        promote_chat_member, set_chat_administrator_custom_title,
        ban_chat_sender_chat, unban_chat_sender_chat, set_chat_permissions,
        export_chat_invite_link, create_chat_invite_link, edit_chat_invite_link,
        revoke_chat_invite_link, set_chat_photo, delete_chat_photo, set_chat_title,
        set_chat_description, pin_chat_message, unpin_chat_message, unpin_all_chat_messages,
        leave_chat, get_chat, get_chat_administrators, get_chat_members_count, get_chat_member_count, get_chat_member,
        set_chat_sticker_set, delete_chat_sticker_set, answer_callback_query,
        set_my_commands, get_my_commands, set_chat_menu_button, get_chat_menu_button, set_my_default_administrator_rights, get_my_default_administrator_rights, delete_my_commands, answer_inline_query, answer_web_app_query, edit_message_text,
        edit_message_text_inline, edit_message_caption, edit_message_caption_inline,
        edit_message_media, edit_message_media_inline, edit_message_reply_markup,
        edit_message_reply_markup_inline, stop_poll, delete_message, send_sticker,
        get_sticker_set, upload_sticker_file, create_new_sticker_set,
        add_sticker_to_set, set_sticker_position_in_set, delete_sticker_from_set,
        set_sticker_set_thumb, send_invoice, answer_shipping_query,
        answer_pre_checkout_query, set_passport_data_errors, send_game,
        set_game_score, set_game_score_inline, get_game_high_scores,
        approve_chat_join_request, decline_chat_join_request => fwd_either, fty_either
    }
}
