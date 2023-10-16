use crate::components::input_component::{Input, InputType};
use yew::prelude::*;

#[function_component(UserRegister)]
pub fn user_register() -> Html {
    let node_ref = use_node_ref();

    html! {
        <div class="register-page">
            <h1>{"Register new user"}</h1>
            <form>
            <Input
                input_node_ref={node_ref.clone()}
                input_placeholder={"user"}
                input_name={"user name"}
                input_type={InputType::Text}/>
            <Input
                input_node_ref={node_ref.clone()}
                input_placeholder={"password"}
                input_name={"user password"}
                input_type={InputType::Password}/>
            </form>
        </div>
    }
}
