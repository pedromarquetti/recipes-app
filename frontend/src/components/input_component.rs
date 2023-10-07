use std::fmt::Display;

use log::debug;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct InputProps {
    pub input_placeholder: String,
    pub input_name: String,
    pub input_type: InputType,
    pub input_node_ref: NodeRef,
}
impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Text => write!(f, "text"),
            Self::Password => write!(f, "password"),
        }
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum InputType {
    Text,
    Password,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let InputProps {
        input_placeholder,
        input_name,
        input_type,
        input_node_ref,
    } = props;
    html! {
        <input
            type={input_type.to_string()}
            name={input_name.clone()}
            placeholder={input_placeholder.clone()}
            ref={input_node_ref.clone()}
        />
    }
}
