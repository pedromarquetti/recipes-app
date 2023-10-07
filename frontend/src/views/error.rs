use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ErrorProps {
    pub text: String,
    pub error_type: ErrorType,
}
#[derive(PartialEq)]
pub enum ErrorType {
    ClientError,
    NotFound,
}

#[function_component(ErrorPage)]
pub fn errors(props: &ErrorProps) -> Html {
    let ErrorProps { text, error_type } = props;
    match error_type {
        ErrorType::ClientError => {
            html! {
                <div class="error-page">
                    <h1>{"An error occurred!"}</h1>
                    <h2>{text}</h2>
                </div>
            }
        }
        ErrorType::NotFound => {
            html! {
                <div class="error-page">
                    <h1>{"404"}</h1>
                    <h2>{text}</h2>
                </div>
            }
        }
    }
}
