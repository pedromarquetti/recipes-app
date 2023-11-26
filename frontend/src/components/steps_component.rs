use db::structs::Step;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StepProps {
    pub step: Step,
}
#[function_component(StepItem)]
/// Represents a base Step item
pub fn step_component(StepProps { step }: &StepProps) -> Html {
    html! {
    <div class="step">
        <h4>{step.step_name.clone()}</h4>
        <p>{format!("{}min - {}",step.step_duration_min,step.step_instruction)}</p>


    </div>
    }
}
