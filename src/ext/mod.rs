//! Deserialize support for serenity-next builder types, folded into poise.

pub mod commands;
pub mod components;
pub mod embed;
pub mod guild;
pub mod interaction;
pub mod message;
pub mod misc;
pub mod modal;
pub mod poll;
pub(crate) mod util;

/// Every wrapper type this crate provides, plus `serde::Deserialize` for
/// generic call sites (`serde_json::from_value::<CreateEmbedDe>(v)` needs
/// only the type in scope, but `fn foo<T: Deserialize>(…)` needs the trait).
pub mod prelude {
    pub use serde::Deserialize;

    pub use crate::ext::commands::CreateCommandDe;
    pub use crate::ext::commands::CreateCommandOptionDe;
    pub use crate::ext::commands::CreateCommandPermissionDe;
    pub use crate::ext::components::CreateActionRowDe;
    pub use crate::ext::components::CreateButtonDe;
    pub use crate::ext::components::CreateComponentDe;
    pub use crate::ext::components::CreateSelectMenuDe;
    pub use crate::ext::components::CreateSelectMenuOptionDe;
    pub use crate::ext::embed::CreateEmbedAuthorDe;
    pub use crate::ext::embed::CreateEmbedDe;
    pub use crate::ext::embed::CreateEmbedFieldDe;
    pub use crate::ext::embed::CreateEmbedFooterDe;
    pub use crate::ext::embed::CreateEmbedImageDe;
    pub use crate::ext::guild::CreateChannelDe;
    pub use crate::ext::guild::CreateForumPostDe;
    pub use crate::ext::guild::CreateForumTagDe;
    pub use crate::ext::guild::CreateInviteDe;
    pub use crate::ext::guild::CreateScheduledEventDe;
    pub use crate::ext::guild::CreateStageInstanceDe;
    pub use crate::ext::guild::CreateThreadDe;
    pub use crate::ext::guild::CreateWebhookDe;
    pub use crate::ext::interaction::AutocompleteChoiceDe;
    pub use crate::ext::interaction::AutocompleteValueDe;
    pub use crate::ext::interaction::CreateAutocompleteResponseDe;
    pub use crate::ext::interaction::CreateInteractionResponseDe;
    pub use crate::ext::interaction::CreateInteractionResponseFollowupDe;
    pub use crate::ext::interaction::CreateInteractionResponseMessageDe;
    pub use crate::ext::message::CreateAllowedMentionsDe;
    pub use crate::ext::message::CreateMessageDe;
    pub use crate::ext::misc::CreateGuildWelcomeChannelDe;
    pub use crate::ext::misc::CreateRoleColoursDe;
    pub use crate::ext::misc::CreateTestEntitlementDe;
    pub use crate::ext::modal::CreateLabelDe;
    pub use crate::ext::modal::CreateModalComponentDe;
    pub use crate::ext::modal::CreateModalDe;
    pub use crate::ext::poll::CreatePollAnswerDe;
    pub use crate::ext::poll::CreatePollDe;
    pub use crate::ext::poll::CreateSoundboardDe;
}
