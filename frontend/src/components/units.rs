use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UnitProps {
    pub id: String,
}
#[function_component(MeasuringUnits)]
pub fn measuring_units(props: &UnitProps) -> Html {
    html! {

    <select required={true}  name="measuring_units" id="measuring_units"
    form={props.id.clone()}
    >
        <option selected={true} disabled={true} value={""}>{"--- Choose a value --- "}</option>
        <option value="teaspoon">{"Teaspoon (tsp)"}</option>
        <option value="tablespoon">{"Tablespoon (tbsp)"}</option>
        <option value="cup">{"Cup"}</option>
        <option value="ounce">{"Ounce (Oz)"}</option>
        <option value="gram">{"Gram (g)"}</option>
        <option value="kilogram">{"Kilogram (Kg)"}</option>
        <option value="liter">{"Liter (L)"}</option>
        <option value="milliliter">{"Milliliter (mL)"}</option>
    </select>
    }
}
