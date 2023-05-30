use std::{rc::Rc, ops::Deref};

use instant::Instant;
use loquat_common::{
    api::fan_series::UpdateBody,
    models::{FanSeries, FanSize, FanType},
};
use web_sys::MouseEvent;
use yew::{
    function_component, html, use_callback, use_state, AttrValue, Callback, Html, Properties,
    UseStateHandle, use_effect_with_deps,
};

use crate::{
    api::store::RequestStatuses,
    common::components::{determination_table::TaggedInput, select::SelectOption, Select},
};

#[derive(Debug, Properties, PartialEq)]
pub struct FanSeriesFormProps {
    pub fan_series_id: Option<AttrValue>,
    #[prop_or_else(|| Rc::new(None))]
    pub maybe_fan_series: Rc<Option<FanSeries<Vec<FanSize<()>>>>>,
    pub on_valid_entry: Callback<UpdateBody>,
    pub on_submit_click: Callback<MouseEvent>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UpdateBodyErrors {
    id_errs: Vec<String>,
    type_errs: Vec<String>,
}

#[function_component]
pub fn FanSeriesForm(
    FanSeriesFormProps {
        fan_series_id,
        maybe_fan_series,
        on_valid_entry,
        on_submit_click,
    }: &FanSeriesFormProps,
) -> Html {
    let fan_series_id_state: UseStateHandle<String> = {
        let fan_series_id = fan_series_id.clone();
        use_state(move || fan_series_id.map_or("".to_string(), |id| id.to_string()))
    };
    let on_fan_series_id_change = {
        let fan_series_id_setter = fan_series_id_state.setter();
        use_callback(
            move |(_index, fan_series_id), _deps| fan_series_id_setter.set(fan_series_id),
            (),
        )
    };
    let fan_type_state: UseStateHandle<Option<FanType>> = use_state(|| None);

    let on_fan_type_select = {
      let fan_type_setter = fan_type_state.setter();
      use_callback(move |value, _| fan_type_setter.set(value), ())
    };

    use_effect_with_deps({
      let fan_series_id_setter = fan_series_id_state.setter();
      let fan_type_setter = fan_type_state.setter();
      move |deps: &Rc<Option<FanSeries<Vec<FanSize<()>>>>>| {
        match deps.as_ref() {
            Some(FanSeries { id, fan_type, fan_sizes }) => {
              fan_series_id_setter.set(id.clone());
              fan_type_setter.set(Some(fan_type.clone()));
            },
            None => {
              fan_series_id_setter.set("".to_string());
              fan_type_setter.set(None);
            },
        }
        
    }}, Rc::clone(&maybe_fan_series));


    let parsed_id = if fan_series_id_state.deref() == "" { Err(vec!["Fan Series ID must be set".to_string()]) } else { Ok(fan_series_id_state.deref())};

    let parsed_type = if let Some(fan_type) = fan_type_state.deref() {
        Ok(fan_type)
    } else {
      Err(vec!["Fan Type must be selected".to_string()])
    };
    let parsed_update_result: Result<UpdateBody, UpdateBodyErrors> = if let (Ok(id), Ok(fan_type)) = (&parsed_id, &parsed_type) {
      Ok(UpdateBody {
        id: (*id).clone(), fan_type: (*fan_type).clone()
      })
    } else {
      Err(UpdateBodyErrors {
        id_errs: parsed_id.err().unwrap_or_default(),
        type_errs: parsed_type.err().unwrap_or_default(),
    })
    };
    
    
    let UpdateBodyErrors {
      id_errs, type_errs
    } = parsed_update_result.err().unwrap_or_default();

    html! {
      <form>
        <div style="display: grid; grid-template-columns: auto auto; width: fit-content; column-gap: 8px; row-gap: 4px;">
            <label>{"Fan Series ID"}</label>
            <TaggedInput<()>
                errs={Rc::new(id_errs)}
                value={(*fan_series_id_state).clone()}
                tag={()}
                onchange={on_fan_series_id_change}
                disabled={fan_series_id.is_some()}
            />
            <label>{"Fan Type"}</label>
            <Select<FanType>
              errs={Rc::new(type_errs)}
              no_selection_label={"--"}
              selection={(*fan_type_state).clone()}
              on_select={on_fan_type_select}
              request_status={Rc::new(RequestStatuses::Fetched(Instant::now()))}
              selectables={Rc::new(FanType::all_options())}
            />
        </div>
        <button onclick={on_submit_click}>
            {"Save"}
        </button>
      </form>
    }
}

impl SelectOption for FanType {
    fn id(&self) -> String {
        self.to_string()
    }
    fn label(&self) -> String {
        self.to_string()
    }
}
