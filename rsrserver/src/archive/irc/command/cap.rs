use std::marker::PhantomData;

use crate::irc::{Anonymous, Authenticated, IrcContext, IrcState, Registered, capability::Caps, command::{HandleRouter, Handler}, error::{IrcResult, IrcSessionError}};
use ircv3_parse::Message;

pub struct Cap<T> {
    _phantom: PhantomData<T>,
}

impl Handler<Anonymous> for Cap<Anonymous> {
    async fn handle<'a>(ctx: IrcContext<'a, Anonymous>, msg: &Message<'_>) -> IrcResult<IrcState> {
        let middles = msg.params().middles;
        match middles.first() {
            Some("LS") => Self::ls(ctx, msg).await,
            Some("LIST") => Self::list(ctx, msg).await,
            Some("REQ") => Self::req(ctx, msg).await,
            Some("END") => Self::end(ctx, msg).await,
            Some(first) => {
                ctx.send_unchecked(&format!(
                    ":* 410 :Invalid CAP command {}\r\n",
                    &first[..first.len().min(470)]
                ))
                .await?;
                Ok(ctx.into())
            }
            None => ctx.send_unchecked(":* 461 :Not enough parameters\r\n").await.map(|_| ctx.into()),
        }
    }
}

impl Handler<Registered> for Cap<Registered> {
    async fn handle<'a>(_ctx: IrcContext<'a, Registered>, _msg: &Message<'_>) -> IrcResult<IrcState> {
        todo!()
    }
}

impl Handler<Authenticated> for Cap<Authenticated> {
    async fn handle<'a>(_ctx: IrcContext<'a, Authenticated>, _msg: &Message<'_>) -> IrcResult<IrcState> {
        todo!()
    }
}

impl<T> Cap<T> where T: Into<IrcState> {
    async fn ls<'a>(mut ctx: IrcContext<'a, T>, msg: &Message<'_>) -> IrcResult<IrcState> {
        if let Some(new_version) = msg.params().middles.second().map(str::parse::<u16>).transpose()? {
            ctx.set_cap_version(new_version);
        }

        let mut chunks = chunk_by_whitespace(Caps::ALL_STR, 490).peekable();

        if ctx.cap_version() >= 302 {
            // multiline send
            while let Some(chunk) = chunks.next() {
                if chunks.peek().is_some() {
                    ctx.send_unchecked("CAP * LS * :").await?;
                }
                else {
                    ctx.send_unchecked("CAP * LS :").await?;
                }

                ctx.send_unchecked(chunk).await?;
                ctx.send_unchecked("\r\n").await?;
            }

            Ok(ctx.into())
        }
        else {
            // server does not accept less than 302 for now.
            ctx.send_unchecked("ERROR :This server does not yet support CAP versions < 302\r\n").await?;
            Err(IrcSessionError::UnsupportedCap)
        }
    }

    async fn list<'a>(mut ctx: IrcContext<'a, T>, msg: &Message<'_>) -> IrcResult<IrcContext<'a, T>> {
        todo!()
    }

    async fn req<'a>(mut ctx: IrcContext<'a, T>, msg: &Message<'_>) -> IrcResult<IrcContext<'a, T>> {
        if let IrcState::Anonymous(anon_state) = ctx.state_mut(){
            anon_state.reg_frozen = true;
        }

        let Some(cap_str) = msg.params().trailing.raw() else {
            ctx.send_unchecked("CAP * ACK :\r\n").await?;
            return Ok(ctx);
        };

        // Negative reqs are TODO
        let (valid, invalid) = Caps::from_string_list(cap_str);

        let valid_str = valid.to_string();
        for chunk in chunk_by_whitespace(&valid_str, 490) {
            ctx.send_unchecked("CAP * ACK :").await?;
            ctx.send_unchecked(chunk).await?;
            ctx.send_unchecked("\r\n").await?;
        }

        ctx.enable_caps(valid);

        // TODO: This joins and then immediately splits
        // the string. This is inefficient and calls for
        // a better abstraction.
        for chunk in chunk_by_whitespace(&invalid.join(" "), 490) {
            ctx.send_unchecked("CAP * NAK :").await?;
            ctx.send_unchecked(chunk).await?;
            ctx.send_unchecked("\r\n").await?;
        }
    
        Ok(ctx)
    }

    async fn end<'a>(mut ctx: IrcContext<'a>, msg: &Message<'_>) -> IrcResult<IrcContext<'a>> {
        // Treat an errant CAP END outside of registration negotiation as harmless and ignore it.
        let IrcState::Anonymous(anon_state) = ctx.state_mut() else {
            return Ok(ctx);
        };

        // If registration negotiation is frozen (by a previous CAP LS or CAP LIST command)
        // then unfreeze it.
        anon_state.reg_frozen = false;

        // Try to register in case this happens to complete the process.
        ctx.try_register().await
    }
}

fn chunk_by_whitespace(text: &str, max_bytes: usize) -> impl Iterator<Item = &str> + '_ {
    let mut pos = 0;
    
    std::iter::from_fn(move || {
        // Skip whitespace
        pos += text[pos..].chars()
            .take_while(|c| c.is_whitespace())
            .map(|c| c.len_utf8())
            .sum::<usize>();
        
        if pos >= text.len() {
            return None;
        }
        
        let rest = &text[pos..];
        
        // If it all fits, we're done
        if rest.len() <= max_bytes {
            pos = text.len();
            return Some(rest);
        }
        
        // Find last whitespace within limit, or first whitespace after
        let split_at = rest.char_indices()
            .take_while(|(i, c)| i + c.len_utf8() <= max_bytes)
            .filter(|(_, c)| c.is_whitespace())
            .last()
            .map(|(i, _)| i)
            .or_else(|| rest.char_indices().find(|(_, c)| c.is_whitespace()).map(|(i, _)| i))
            .unwrap_or(rest.len());
        
        let chunk = &rest[..split_at];
        pos += split_at;
        Some(chunk)
    })
}
