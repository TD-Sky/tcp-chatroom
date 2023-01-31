mod components;
mod message;
mod port;
mod view;

use self::{
    components::{ChatList, ComponentId},
    message::Msg,
    port::HorzListener,
};
use crate::{models::Group, Config};
use anyhow::{Context, Result};
use duang::Duang;
use indexmap::IndexMap;
use protocol::persistent::Horz;
use std::{
    collections::{HashMap, HashSet},
    io::{self, BufWriter, Write},
    net::TcpStream,
    time::Duration,
};
use tuirealm::{
    application::ApplicationResult,
    terminal::{TerminalBridge, TerminalResult},
    Application, EventListenerCfg,
};

static REDRAW_INTERVAL: Duration = Duration::from_millis(100);

pub struct Ui {
    /// Application
    pub app: Application<ComponentId, Msg, Horz>,
    /// 封装的TUI后端
    pub terminal: TerminalBridge,

    /// 配置
    pub config: Config,
    /// 鉴权Token，含有用户ID信息
    pub token: String,

    /// 长连接通信的写权柄
    pub writer: BufWriter<TcpStream>,

    /// 用户所在的群组
    pub my_groups: Vec<Group>,
    /// ID-名字 查询
    pub id2name: HashMap<i64, String>,
    /// 聊天对象，全服聊天、群组、用户
    pub chats: IndexMap<i64, String>,
    /// 正在聊天的ID
    pub chatting: i64,
    /// 消息提示的ID组
    pub beeps: HashSet<i64>,
}

impl Ui {
    pub fn init(config: Config, token: String, socket: TcpStream) -> Result<Self> {
        let app = Application::init(
            EventListenerCfg::default()
                .default_input_listener(REDRAW_INTERVAL)
                .poll_timeout(REDRAW_INTERVAL)
                .port(
                    Box::new(HorzListener::new(socket.try_clone()?)),
                    REDRAW_INTERVAL,
                ),
        );
        let terminal = TerminalBridge::new()?;

        let address = config.address.as_str();

        let my_groups: Vec<Group> = Duang::builder(address)
            .token(&token)
            .try_build()?
            .send()?
            .data()
            .context("could not fetch user's groups")?;

        let chats = IndexMap::from([(0, "全服聊天".to_owned())]);
        let id2name: HashMap<i64, String> = Duang::builder(address)
            .token(&token)
            .try_build()?
            .send()?
            .data()
            .context("could not fetch id-name map")?;

        let mut ui = Self {
            app,
            terminal,
            config,
            token,
            writer: BufWriter::new(socket),
            my_groups,
            id2name,
            chats,
            chatting: 0,
            beeps: HashSet::new(),
        };

        ui.init_application()?;
        ui.init_terminal()?;

        Ok(ui)
    }

    pub fn run() {
        todo!()
    }
}

impl Ui {
    fn init_application(&mut self) -> ApplicationResult<()> {
        self.app
            .mount(ComponentId::ChatList, Box::<ChatList>::default(), vec![])?;
        self.app.active(&ComponentId::ChatList)?;

        Ok(())
    }

    fn init_terminal(&mut self) -> TerminalResult<()> {
        self.terminal.enable_raw_mode()?;
        self.terminal.enter_alternate_screen()?;
        self.terminal.clear_screen()?;

        Ok(())
    }

    fn write_horz(&mut self, horz: Horz) -> io::Result<()> {
        self.writer
            .write_all(horz.method().to_string().as_bytes())?;
        self.writer.write_all(b"\n")?;

        if let Some(data) = horz.bytes() {
            self.writer
                .write_all(format!("Length = {}\n", data.len() as u64).as_bytes())?;
            self.writer.write_all(data)?;
        } else {
            self.writer.write_all(b"Length = 0\n")?;
        }

        Ok(())
    }
}
