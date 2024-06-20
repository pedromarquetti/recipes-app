use db::structs::User;
use log::{error, info};
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_notifications::{use_notification, Notification};
use yew_router::hooks::use_navigator;

use crate::{
    components::input_component::{Input, InputType},
    functions::{user_functions::login_user, ApiResponse},
    views::Route,
    DEFAULT_NOTIFICATION_DURATION,
};

#[function_component(UserLogin)]
pub fn user_login() -> Html {
    let use_notification = use_notification::<Notification>();
    let user_name_ref = use_node_ref();
    let user_pwd_ref = use_node_ref();
    let navigator = use_navigator().unwrap();

    let onsubmit = {
        let user_name_ref = user_name_ref.clone();
        let user_pwd_ref = user_pwd_ref.clone();
        Callback::from(move |event: SubmitEvent| {
            let navigator = navigator.clone();
            event.prevent_default();
            let use_notification = use_notification.clone();

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

            let mut usr = User::default();
            usr.user_name = user_name_ref;
            usr.user_pwd = user_pwd_ref;

            spawn_local(async move {
                match login_user(usr).await {
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
                            info!("API message: {:?}", msg);
                            use_notification.spawn(Notification::new(
                                yew_notifications::NotificationType::Info,
                                "Sucess",
                                msg,
                                DEFAULT_NOTIFICATION_DURATION,
                            ));

                            navigator.push(&Route::Home);
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
        <div class="login-page">
            <h1>{"User Login"}</h1>

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
            <button>{"Login"}</button>
            </form>
        </div>
    }
}
