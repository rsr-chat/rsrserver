macro_rules! caps {
    // Entry point - start recursion with index 0
    [$(($cap:expr, $name:ident)),+ $(,)?] => {
        caps!(@impl 0; ; ; ; $(($cap, $name),)+);
    };
    
    // Recursive case: process one item at a time
    (@impl $index:expr; 
     $(($acc_cap:expr, $acc_name:ident, $acc_idx:expr),)*; 
     $(($impl_cap:expr, $impl_name:ident),)*;
     $(($str_cap:expr),)*;
     ($cap:expr, $name:ident), $($rest:tt)*
    ) => {
        caps!(@impl $index + 1;
              $(($acc_cap, $acc_name, $acc_idx),)* ($cap, $name, $index),;
              $(($impl_cap, $impl_name),)* ($cap, $name),;
              $(($str_cap),)* ($cap),;
              $($rest)*
        );
    };
    
    // Base case: no more items, generate the output
    (@impl $index:expr;
     $(($acc_cap:expr, $acc_name:ident, $acc_idx:expr),)*;
     $(($impl_cap:expr, $impl_name:ident),)*;
     $(($str_cap:expr),)*;
    ) => {
        bitflags::bitflags! {
            pub struct Capabilities: u64 {
                $(
                    const $acc_name = 1 << $acc_idx;
                )*
            }
        }

        pub const ALL_CAPS: &str = concat!(
            $($str_cap, " "),*
        );
    };
}

caps![
    ("account-notify", CapAccountNotify),
    ("account-tag", CapAccountTag),
    ("away-notify", CapAwayNotify),
    ("batch", CapBatch),
    ("channel-rename", CapChannelRename),
    ("chathistory", CapChatHistory),
    ("echo-message", CapEchoMessage),
    ("extended-join", CapExtendedJoin),
    ("labeled-response", CapLabeledResponse),
    ("message-redaction", CapMessageRedaction),
    ("message-tags", CapMessageTags),
    ("monitor", CapMonitor),
    ("multi-prefix", CapMultiPrefix),
    ("multiline", CapMultiline),
    ("pre-away", CapPreAway),
    ("read-marker", CapReadMarker),
    ("sasl", CapSasl),
    ("server-time", CapServerTime),
    ("standard-replies", CapStandardReplies),
    ("userhost-in-names", CapUserhostInNames),
    ("rsr.chat/massive-message", CapRsrvcMassiveMessage),   // - Message body up to 2048 bytes from 512.
    ("rsr.chat/plc-oauthbearer", CapRsrvcPlcOauthbearer),   // - Extension to SASL OAUTHBEARER
                                                            //   that supports PLC lookups via DID.
    ("rsr.chat/did-signing", CapRsrvcDidSigning),   // - DID message signing tags
    ("rsr.chat/moderation", CapRsrvcModeration),    // - Advanced moderation tools imported
                                                    //   from other platforms, with interop
                                                    //   with other rsr.chat extensions.
    ("rsr.chat/rbac", CapRsrvcRbac),                // - Role based ACLs for users.
    ("rsr.chat/onboarding", CapRsrvcOnboarding),    // - Tools for per-server onboarding flow
    ("rsr.chat/external-notify", CapRsrvcExternalNotify),   // - Account-notify, away-notify, etc
                                                            //   backed by decentralized PLC method.
    ("rsr.chat/pending-messages", CapRsrvcPendingMessages), // - MARKREAD extension that allows
                                                            //   simple queries to see if a channel
                                                            //   has unread messages waiting.
    ("rsr.chat/pins", CapRsrvcPins),                        // - Support for channel-wide saved
                                                            //   pinned message lists.
    ("rsr.chat/message-revision", CapRsrvcMessageRevision), // - Support for "message edits" which
                                                            //   may be configured to have their edit
                                                            //   history remain visible or invisible.
    ("rsr.chat/message-link", CapRsrvcMessageLink),         // - Support links to other messages.
    ("rsr.chat/message-reply", CapRsrvcMessageReply),       // - Support messages marked as replies
                                                            //   to other messages.
    ("rsr.chat/account-profile", CapRsrvcAccountProfile),   // - Support for account bios, statuses,
                                                            //   tags, etc. over PDS.
    ("rsr.chat/modern-ping", CapRsrvcModernPing),           // - Support for @USER, @HERE, @EVERYONE
                                                            //   and (if RBAC is enabled) @role pings.
    ("rsr.chat/react", CapRsrvcReact),  // - Support for message reactions.
    ("rsr.chat/emote", CapRsrvcEmote),  // - Support for custom server emoticons.
    ("rsr.chat/sticker", CapRsrvcSticker),  // - Support for custom server stickers.
    ("rsr.chat/server-meta", CapRsrvcServerMeta),   // - Generalized extension for adding
                                                    //   metadata to servers as a whole.
    ("rsr.chat/channel-meta", CapRsrvcChannelMeta), // - Generalized extension for adding
                                                    //   metadata to server channels.
    ("rsr.chat/channel-category", CapRsrvcChannelCategory), // - Builds upon rsr.chat/channel-meta
                                                            //   to provide sorted channel categories.
    ("rsr.chat/channel-nsfw", CapRsrvcChannelNsfw),         // - Support for server content tagging
                                                            //   that can be used to restrict access
                                                            //   to adults, with various levels of
                                                            //   strictness and verification.
    ("rsr.chat/voice", CapRsrvcVoice),  // - Support for real time voice chat.
    ("rsr.chat/video", CapRsrvcVideo),  // - Support for real time video chat.
];