#[derive(Clone, PartialEq, Eq)]
pub enum Msg {
    SwitchChatRoom,
    SwitchChatList,
    ChatList(ChatListMsg),
}

#[derive(Clone, PartialEq, Eq)]
pub enum ChatListMsg {
    Select(usize),
    Beep(i64),
}
