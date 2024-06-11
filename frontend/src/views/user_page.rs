use super::{UserLogin, UserRegister};
use yew::prelude::*;

enum UserPageState {
    Login,
    Register,
}

#[function_component(UserPage)]
pub fn user_page() -> Html {
    let state = use_state(|| UserPageState::Login);

    let set_login = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(UserPageState::Login);
        })
    };
    let set_register = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(UserPageState::Register);
        })
    };
    return html! {
        <div class="user-auth" >
            <h1>{"User Login/Register"}</h1>


            <div class="user-auth-buttons">
                <button
                    onclick={set_login}
                    // class={if matches!(*state, UserPageState::Login) { "active" } else { "" }}
                >
                    {"Login"}
                </button>
                <button
                    onclick={set_register}
                    // class={if matches!(*state, UserPageState::Register) { "active" } else { "" }}
                >
                    {"Register"}
                </button>
            </div>
            {
                match *state.clone() {
                    UserPageState::Login => html! {
                        <UserLogin />
                    },
                    UserPageState::Register => html! {
                        <UserRegister />
                    },
                }
            }
        </div>
    };
}
