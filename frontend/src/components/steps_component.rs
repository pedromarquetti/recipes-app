use db::structs::Step;
use log::debug;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StepListProps {
    pub steps: Vec<Step>,
}
#[function_component(StepsList)]
pub fn steps_list(StepListProps { steps }: &StepListProps) -> Html {
    let l: Vec<Html> = steps
        .iter()
        .map(|step: &Step| {
            html! {
                <li id={format!("{}",step.id.unwrap())} class="step">
                <h2>{step.step_name.clone()}</h2>
                <p>{format!("{}min - {}",step.step_duration_min,step.step_instruction)}</p>

                </li>
            }
        })
        .collect();

    html! {
    <div class="steps-list">
        <h2 >{"Steps"}</h2>
        <ol>
            {l}
        </ol>
    </div>
    }
}
