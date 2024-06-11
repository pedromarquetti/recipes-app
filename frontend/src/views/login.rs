use yew::prelude::*;

use crate::components::input_component::{Input, InputType};

#[function_component(UserLogin)]
pub fn user_login() -> Html {
    let node_ref = use_node_ref();

    html! {
        <div class="login-page">
            <h1>{"User Login"}</h1>
            <form>
            <Input
                input_node_ref={node_ref.clone()}
                input_placeholder={"user"}
                is_required={true}

                input_name={"user name"}
                input_type={InputType::Text}/>
            <Input
                input_node_ref={node_ref.clone()}
                input_placeholder={"password"}
                input_name={"user password"}
                is_required={true}

                input_type={InputType::Password}/>
            <button>{"Login"}</button>
            </form>
        </div>
    }
}
