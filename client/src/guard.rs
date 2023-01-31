use crate::models::{LoginUser, RegisterUser};
use requestty::{prompt, Answers, Question};

pub fn manually_login() -> requestty::Result<LoginUser> {
    let questions = [
        Question::int("id").message("用户ID").build(),
        Question::input("nickname")
            .message("昵称")
            .validate(non_empty)
            .build(),
        Question::password("password")
            .message("密码")
            .validate(validate_password)
            .build(),
    ];

    let mut answers = prompt(questions)?;

    // 因为设置了校验器，此处的 unwrap 必定成功
    let user = LoginUser {
        id: answers.get("id").and_then(|a| a.as_int()).unwrap(),
        nickname: answers
            .remove("nickname")
            .and_then(|a| a.try_into_string().ok())
            .unwrap(),
        password: answers
            .remove("password")
            .and_then(|a| a.try_into_string().ok())
            .unwrap(),
    };

    Ok(user)
}

pub fn register_login() -> requestty::Result<RegisterUser> {
    let questions = [
        Question::input("nickname")
            .message("昵称")
            .validate(non_empty)
            .build(),
        Question::input("password")
            .message("密码")
            .validate_on_key(validate_char)
            .validate(validate_password)
            .build(),
    ];

    let mut answers = prompt(questions)?;

    // 因为设置了校验器，此处的 unwrap 必定成功
    let user = RegisterUser {
        nickname: answers
            .remove("nickname")
            .and_then(|a| a.try_into_string().ok())
            .unwrap(),
        password: answers
            .remove("password")
            .and_then(|a| a.try_into_string().ok())
            .unwrap(),
    };

    Ok(user)
}

/// 输入即时校验
fn validate_char(password: &str, _: &Answers) -> bool {
    password.contains(char::is_alphanumeric) || password.contains(['+', '-', '*', '/', '?', '!'])
}

/// 输入完成校验
fn validate_password(password: &str, pre_answers: &Answers) -> Result<(), String> {
    if !(8..=20).contains(&password.len()) {
        return Err("Password should have at least 8 and at most 20 characters".to_owned());
    }

    if !validate_char(password, pre_answers) {
        return Err("Invalid password".to_owned());
    }

    Ok(())
}

/// 昵称不能为空
fn non_empty(nickname: &str, _: &Answers) -> Result<(), String> {
    if nickname.is_empty() {
        Err("Nickname is empty".to_owned())
    } else {
        Ok(())
    }
}
