use crate::ui::{
    Ui,
    components::ComponentId,
    message::{ChatListMsg, Msg},
};
use protocol::{
    models::{GroupMessage, PrivateMessage},
    persistent::{Horz, Method},
};
use tui_realm_stdlib::Table;
use tuirealm::{
    application::ApplicationResult,
    command::{Cmd, Direction},
    event::{Key, KeyEvent, KeyModifiers},
    props::{Alignment, Color, TableBuilder, TextSpan},
    AttrValue, Attribute, Component, Event, MockComponent,
};

#[derive(MockComponent)]
pub struct ChatList {
    component: Table,
}

impl Default for ChatList {
    fn default() -> Self {
        Self {
            component: Table::default()
                .scroll(true)
                .title("聊天列表", Alignment::Left)
                .foreground(Color::White)
                .highlighted_str("\u{1f9d0}") // 🧐
                .highlighted_color(Color::Yellow)
                .widths(&[30, 4])
                .table(
                    TableBuilder::default()
                        .add_row()
                        .add_col(TextSpan::new("全服聊天"))
                        .add_col(TextSpan::new(" "))
                        .add_col(TextSpan::new(" "))
                        .build(),
                )
                .selected_line(0),
        }
    }
}

impl Component<Msg, Horz> for ChatList {
    fn on(&mut self, ev: Event<Horz>) -> Option<Msg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => match ch {
                'k' => Cmd::Move(Direction::Up),
                'j' => Cmd::Move(Direction::Down),
                'l' => {
                    return Some(Msg::ChatList(ChatListMsg::Select(
                        self.component.states.list_index,
                    )))
                }
                _ => Cmd::None,
            },

            Event::Keyboard(KeyEvent {
                code: Key::Tab,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::SwitchChatRoom),

            Event::User(horz) => {
                let id = extract_id(horz)?;
                return Some(Msg::ChatList(ChatListMsg::Beep(id)));
            }

            _ => Cmd::None,
        };

        self.perform(cmd);

        None
    }
}

fn extract_id(horz: Horz) -> Option<i64> {
    let id = match horz.method() {
        Method::PublicMessage => 0,
        Method::GroupMessage => {
            let gmsg: GroupMessage = horz.data().unwrap();
            gmsg.gid as i64
        }
        Method::PrivateMessage => {
            let pmsg: PrivateMessage = horz.data().unwrap();
            pmsg.uid
        }
        _ => return None,
    };

    Some(id)
}

impl Ui {
    pub fn update_chat_list(&mut self) -> ApplicationResult<()> {
        let mut table = TableBuilder::default();

        for (&id, name) in self.chats.iter() {
            table
                .add_row()
                .add_col(TextSpan::new(name))
                .add_col(TextSpan::new(" ").fg(if id == self.chatting {
                    Color::Red
                } else {
                    Color::Reset
                }));
        }

        let table = table.build();
        self.app.attr(
            &ComponentId::ChatList,
            Attribute::Content,
            AttrValue::Table(table),
        )
    }
}
