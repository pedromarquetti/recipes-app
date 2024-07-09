use db::structs::Step;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StepProps {
    pub step: Step,
    #[prop_or(false)]
    pub edit_mode: bool,
    #[prop_or_default]
    pub curr_focus: Callback<Step>,
}
#[function_component(StepItem)]
/// Represents a base Step item
pub fn step_component(prop: &StepProps) -> Html {
    let StepProps {
        step,
        edit_mode,
        curr_focus,
    } = prop;
    let edit_mode = edit_mode.clone();
    let focus_state = use_state(|| false);

    html! {
    <div
        onmouseleave={{
            let focus_state = focus_state.clone();
            Callback::from(move |_| {
            focus_state.set(false)
        })}}
        onmouseenter={{
            let focus_state = focus_state.clone();
            Callback::from(move |_| {
            focus_state.set(true)
        })}} class="step">
        {
        if edit_mode && *focus_state {
        html!{
        <button onclick={{
            let step = step.clone();
            let curr_focus = curr_focus.clone();
            Callback::from(move |_| {
                let step = step.clone();
                curr_focus.emit(step)
            })
        }}>{"Edit"}</button>
    }} else {html!{<></>}}
        }
        <h4>{step.step_name.clone()}</h4>
        <p>{format!("{}min - {}",step.step_duration_min,step.step_instruction)}</p>


    </div>
    }
}
