mod chat_list;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ComponentId {
    ChatList,
}

pub use self::chat_list::ChatList;
