# Leptos Controls

在[Leptos](https://github.com/leptos-rs/leptos)中对结构体进行整体控制的工具包。

## Examples

```rust

use leptos::*;
use leptos_controls::Controls;

#[derive(Default, Controls)]
pub struct LoginArgs {
    #[field(label = "账号", validate = "is_not_empty", message = "账号不能为空")]
    account: String,

    #[field(label = "密码", validate = "is_not_empty", message = "密码不能为空")]
    password: String,
}

fn is_not_empty(text: &str) -> bool {
    !text.is_empty()
}

fn example() {
    let args = LoginArgs {
        account: "admin".to_string(),
        password: "123456".to_string(),
    };
    // create controls
    let controls = LoginArgsControls::new(args);

    // reactive type (RwSignal)
    let account = controls.account;
    let password =controls.password;

    // get field label
    let label = controls.account.label();

    // get field required
    let account_required = controls.account.required();

    // get controls validate errors
    let errors = controls.validate();

    // set signal field default value
    controls.account.set_default();

    // set all field default value
    controls.set_default();

    // get snapshot args
    let args = controls.snapshot();
}

```

## License

This project is licensed under the [Apache 2.0](./LICENSE)