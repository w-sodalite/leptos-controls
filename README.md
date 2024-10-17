# Leptos Controls

[![Crates.io][crates-badge]][crates-url]
[![Apache licensed][apache-badge]][apache-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/leptos-controls.svg
[crates-url]: https://crates.io/crates/leptos-controls
[apache-badge]: https://img.shields.io/badge/license-Aapche-blue.svg
[apache-url]: LICENSE
[actions-badge]: https://github.com/w-sodalite/leptos-controls/workflows/Rust/badge.svg
[actions-url]: https://github.com/w-sodalite/leptos-controls/actions?query=workflow%3ARust

在[Leptos](https://github.com/leptos-rs/leptos)中对结构体进行整体控制,支持配置字段名称、校验、定义错误信息等通用逻辑。

## Usage

```toml
leptos-controls = { version = "0.1.6" }
```

use [thaw](https://github.com/thaw-ui/thaw)

```toml
leptos-controls = { version = "0.1.6", features = ["thaw"] }
```

## Examples

```rust
#[component]
pub fn New(show: RwSignal<bool>, action: ChargeAction<NewFn>) -> impl IntoView {
    let message = use_message();
    let args = NewUserArgs::default();
    let controls = NewUserArgsControls::new(args);
    create_effect(move |prev| {
        // 关闭时重置数据
        if prev.is_some() && !show.get() {
            controls.set_default();
        }
        show.get()
    });

    move || {
        view! {
            <Modal show title="新增">
                <Space vertical=true>
                    <FieldLabel field=controls.name>
                        <Input value=controls.name />
                    </FieldLabel>
                    <FieldLabel field=controls.account>
                        <Input value=controls.account />
                    </FieldLabel>
                    <FieldLabel field=controls.password>
                        <Input value=controls.password variant=InputVariant::Password />
                    </FieldLabel>
                    <FieldLabel field=controls.mobile>
                        <Input value=controls.mobile variant=InputVariant::Password />
                    </FieldLabel>
                    <FieldLabel field=controls.email>
                        <Input value=controls.email variant=InputVariant::Password />
                    </FieldLabel>

                    <Divider />

                    <Space justify=SpaceJustify::End>
                        <Button
                            color=ButtonColor::Primary
                            on_click=move |_| {
                                let errors = controls.validate();
                                if !errors.is_empty() {
                                    message.error(errors.join("|"));
                                } else {
                                    let args = controls.snapshot();
                                    action.dispatch(args.into());
                                    show.set(false);
                                }
                            }
                        >
                            新增
                        </Button>
                        <Button
                            variant=ButtonVariant::Outlined
                            on_click=move |_| {
                                show.set(false);
                            }
                        >
                            取消
                        </Button>
                    </Space>
                </Space>
            </Modal>
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Controls)]
struct NewUserArgs {
    #[field(
        label = "名称",
        validate = "crate::util::is_not_blank",
        message = "名称不能为空"
    )]
    name: String,

    #[field(
        label = "账号",
        validate = "crate::util::is_not_blank",
        message = "账号不能为空"
    )]
    account: String,

    #[field(
        label = "密码",
        validate = "crate::util::is_not_blank",
        message = "密码不能为空"
    )]
    password: String,

    #[field(label = "手机号码")]
    mobile: String,

    #[field(label = "邮箱")]
    email: String,
}

#[server(name=NewFn, client=AuthorizationClient)]
#[cfg_attr(feature = "ssr", tracing::instrument(name = "NewFn", level = "info"))]
async fn new(args: NewUserArgs) -> Result<(), ServerFnError> {
    use crate::ssr::*;
    let NewUserArgs {
        name,
        account,
        password,
        mobile,
        email,
        ..
    } = args;
    let state = expect_context::<State>();
    let password = sha256(&password);
    users::Entity::insert(users::ActiveModel {
        name: Set(name),
        account: Set(account),
        password: Set(password),
        status: Set(Status::On),
        mobile: Set(Some(mobile)),
        email: Set(Some(email)),
        ..Default::default()
    })
        .exec(&state)
        .await
        .map_err(ServerFnError::new)
        .map(|_| ())
}

```

## License

This project is licensed under the [Apache 2.0](./LICENSE)