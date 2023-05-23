use std::rc::Rc;

use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_callback, use_effect_with_deps, use_node_ref, Callback, Html,
    Properties,
};

use crate::api::store::RequestStatuses;

pub trait SelectOption {
    fn id(&self) -> String;
    fn label(&self) -> String;
}

#[derive(Properties, PartialEq)]
pub struct SelectProps<T: PartialEq> {
    #[prop_or(vec![])]
    pub errs: Vec<String>,
    pub no_selection_label: String,
    pub selection: Option<T>,
    pub on_select: Callback<Option<T>, ()>,
    pub request_status: Rc<RequestStatuses>,
    pub selectables: Rc<Vec<T>>,
}

#[function_component]
pub fn Select<T: PartialEq + Clone + SelectOption + 'static>(
    SelectProps {
        errs,
        no_selection_label,
        selection,
        on_select,
        request_status,
        selectables,
    }: &SelectProps<T>,
) -> Html {
    let select_ref = use_node_ref();

    let selection_id = selection.as_ref().map_or("".to_string(), |s| s.id());

    use_effect_with_deps(
        move |(selection_id_ref, select_ref_dep)| {
            let select_el = select_ref_dep
                .cast::<HtmlInputElement>()
                .expect("select_ref not attached to select element");
            select_el.set_value(&selection_id_ref[..]);
        },
        (selection_id.clone(), select_ref.clone()),
    );

    let selected_option: Html = match selection {
        Some(s) => html! {
          <option value={s.id()}>
            {s.label()}
          </option>
        },
        None => html! {
          <option value={""}>
            {no_selection_label}
          </option>
        },
    };

    let all_options: Html = selectables
        .iter()
        // .filter(|opt| opt.id() != selection_id)
        .map(|s| {
            html!(
              <option value={s.id()}>
                {s.label()}
              </option>
            )
        })
        .collect::<Html>();

    if let Some(input) = select_ref.cast::<HtmlInputElement>() {
        if errs.is_empty() {
            input.set_custom_validity("");
        } else {
            input.set_custom_validity(&errs.join("\n"));
        }
        input.report_validity();
    }

    let select_callback = use_callback(
        move |_evt: web_sys::Event, (on_select_dep, select_ref_dep, fan_serieses_ref)| {
            let select_el = select_ref_dep
                .cast::<HtmlInputElement>()
                .expect("select_ref not attached to select element");

            let selected_id: String = select_el.value();
            let selection: Option<T> = fan_serieses_ref
                .iter()
                .find(|fs| selected_id.eq(&fs.id()))
                .cloned();
            on_select_dep.emit(selection);
        },
        (on_select.clone(), select_ref.clone(), selectables.clone()),
    );

    let options: Html = match request_status.as_ref() {
        RequestStatuses::Error(_error_at, _msgg) => {
            html! {
              <>
                {selected_option}
                <option>{"Error loading items..."}</option>
              </>
            }
        }
        RequestStatuses::Unfetched => {
            html! {<>
                {selected_option}
                <option>{"Loading..."}</option>
                </>
            }
        }
        RequestStatuses::Fetching(_fetched_at) => {
            html! {
              <>
                {selected_option}
                <option>{"Loading..."}</option>
              </>
            }
        }
        RequestStatuses::Refetching(_fetched_at, _last_status) => {
            html! {
              <>
                <option>{"Loading..."}</option>
                {all_options}
              </>
            }
        }

        RequestStatuses::Fetched(_fetched_at) => {
            html! {
              <>
                {all_options}
              </>
            }
        }
    };

    let style: &str = if !errs.is_empty() {
        "border: 1px solid red;"
    } else {
        ""
    };

    html! {
      <select {style} onchange={select_callback} ref={select_ref}>
        {options}
      </select>
    }
}
