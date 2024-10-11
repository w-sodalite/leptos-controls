use leptos::*;
use leptos_controls::{Controls, Field};

fn is_not_empty(text: &str) -> bool {
    !text.is_empty()
}

#[derive(Debug, Default, Controls)]
pub struct LoginArgs {
    #[field(label = "账号", validate = "is_not_empty", message = "账号不能为空")]
    account: String,

    #[field(label = "密码", readonly = true)]
    password: String,
}

fn main() {
    let args = LoginArgs::default();
    let controls = LoginArgsControls::new(args);
    let errors = controls.validate();
    println!("{:?}", errors);
    // ["账号不能为空"]

    controls.account.set("admin".to_string());
    controls.account.set("123456".to_string());

    let errors = controls.validate();
    println!("{:?}", errors);
    // []

    controls.account.validate();

    let account = controls.account.label();
    println!("{}", account);
    // 账号

    let password = controls.password.label();
    println!("{}", password);
    // 密码

    let required = controls.account.required();
    println!("{}", required);
    // true

    let required = controls.password.required();
    println!("{}", required);
    // false

    let args = controls.snapshot();
    println!("{:?}", args);
    // LoginArgs {account: "admin", password: "123456"}

    // set default
    controls.account.set_default();
    controls.set_default();
}
