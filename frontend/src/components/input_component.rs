use std::fmt::Display;

use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct InputProps {
    pub input_placeholder: String,
    pub input_name: String,
    pub input_type: InputType,
    pub input_node_ref: NodeRef,
    pub is_required: bool,
}
impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Text => write!(f, "text"),
            Self::Password => write!(f, "password"),
            Self::Number => write!(f, "number"),
        }
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum InputType {
    Text,
    Password,
    Number,
}

#[function_component(Input)]
/// \<Input\/\> component
/// # Params:
/// input_placeholder -> what will be shown in the Input element as a placeholder
///
/// input_name -> \<input\> name (duh)
///
/// input_type -> InputType enum `usecrate::components::input_component::InputType`)
///
/// input_node_ref -> uses use_node_ref(). Used for event handling with CallBack (https://yew.rs/docs/concepts/html/events#using-noderef)
///
pub fn input(props: &InputProps) -> Html {
    let InputProps {
        input_placeholder,
        input_name,
        input_type,
        input_node_ref,
        is_required,
    } = props;
    html! {
        <input
            type={input_type.to_string()}
            name={input_name.clone()}
            placeholder={input_placeholder.clone()}
            ref={input_node_ref.clone()}
            required={is_required.clone()}

        />
    }
}
