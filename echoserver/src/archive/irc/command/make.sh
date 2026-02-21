#!/usr/bin/env zsh

names=(
    authenticate
    pass
    nick
    user
    ping
    pong
    oper
    quit
    error
    join
    part
    topic
    names
    list
    invite
    kick
    motd
    version
    admin
    connect
    lusers
    time
    stats
    help
    info
    mode
    privmsg
    notice
    who
    whois
    whowas
    kill
    rehash
    restart
    squit
    away
    links
    userhost
    wallops
)

# Function to convert snake_case to PascalCase
to_pascal_case() {
    local input="$1"
    local result=""
    
    # Split by underscore and capitalize each part
    for word in ${(s:_:)input}; do
        result+="${(U)word[1]}${word[2,-1]}"
    done
    
    echo "$result"
}

for name in $names; do
    pascal_name=$(to_pascal_case "$name")
    filename="${name}.rs"
    
    cat > "$filename" <<EOF
use std::marker::PhantomData;
use crate::irc::{Anonymous, Authenticated, IrcContext, IrcState, Registered, command::Handler, error::IrcResult};
use ircv3_parse::Message;

pub struct ${pascal_name}<T> {
    _phantom: PhantomData<T>
}

impl Handler<Anonymous> for ${pascal_name}<Anonymous> {
    async fn handle<'a>(_ctx: IrcContext<'a, Anonymous>, _msg: &Message<'_>) -> IrcResult<IrcState> {
        todo!()
    }
}

impl Handler<Registered> for ${pascal_name}<Registered> {
    async fn handle<'a>(_ctx: IrcContext<'a, Registered>, _msg: &Message<'_>) -> IrcResult<IrcState> {
        todo!()
    }
}

impl Handler<Authenticated> for ${pascal_name}<Authenticated> {
    async fn handle<'a>(_ctx: IrcContext<'a, Authenticated>, _msg: &Message<'_>) -> IrcResult<IrcState> {
        todo!()
    }
}

EOF
    
    echo "Created $filename with struct $pascal_name"
done