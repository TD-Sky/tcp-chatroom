use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// 手动填写登录信息
    #[arg(long, short, conflicts_with = "register_login")]
    pub manually_login: bool,

    /// 注册新用户并登录
    #[arg(long, short, conflicts_with = "manually_login")]
    pub register_login: bool,
}
