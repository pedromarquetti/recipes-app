use db::structs::NewUser;
use log::error;
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_notifications::{use_notification, Notification};

use crate::{
    components::input_component::{Input, InputType},
    functions::{user_functions::create_user, ApiResponse},
    DEFAULT_NOTIFICATION_DURATION,
};

#[function_component(UserRegister)]
pub fn user_register() -> Html {
    let use_notification = use_notification::<Notification>();
    let user_name_ref = use_node_ref();
    let user_pwd_ref = use_node_ref();

    let onsubmit = {
        let user_name_ref = user_name_ref.clone();
        let user_pwd_ref = user_pwd_ref.clone();
        Callback::from(move |event: SubmitEvent| {
            let use_notification = use_notification.clone();
            event.prevent_default();

            let user_name_ref = user_name_ref
                .clone()
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();
            let user_pwd_ref = user_pwd_ref
                .clone()
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();

            let mut usr = NewUser::default();
            usr.user_name = user_name_ref;
            usr.user_pwd = user_pwd_ref;

            spawn_local(async move {
                match create_user(usr).await {
                    Ok(ok_login) => match ok_login {
                        ApiResponse::ApiError(err) => {
                            error!("API error: {:?}", err);
                            use_notification.spawn(Notification::new(
                                yew_notifications::NotificationType::Error,
                                "Error!",
                                err,
                                DEFAULT_NOTIFICATION_DURATION,
                            ));
                        }
                        ApiResponse::ApiMessage(msg) => {
                            use_notification.spawn(Notification::new(
                                yew_notifications::NotificationType::Info,
                                "Sucess",
                                msg,
                                DEFAULT_NOTIFICATION_DURATION,
                            ));
                        }
                        _ => {} // this is a placeholder
                    },
                    Err(err) => {
                        error!("error: {:?}", err);
                        use_notification.spawn(Notification::new(
                            yew_notifications::NotificationType::Error,
                            "Error!",
                            err.to_string(),
                            DEFAULT_NOTIFICATION_DURATION,
                        ));
                    }
                }
            })
        })
    };

    html! {
        <div class="register-page">
            <h1>{"Register new user"}</h1>
            <form {onsubmit}>
            <Input
                input_node_ref={user_name_ref.clone()}
                input_placeholder={"username"}
                is_required={true}

                input_name={"user name"}
                input_type={InputType::Text}/>
            <Input
                input_node_ref={user_pwd_ref.clone()}
                input_placeholder={"password"}
                input_name={"user password"}
                is_required={true}

                input_type={InputType::Password}/>
            <button>{"Register"}</button>
            </form>
        </div>
    }
}
