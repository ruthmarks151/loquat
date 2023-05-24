use std::rc::Rc;

use loquat_common::models::FanSize;
use yew::{function_component, html, use_effect_with_deps, AttrValue, Callback, Html, Properties};
use yewdux::prelude::{use_selector_with_deps, use_store};

use crate::api::store::Store as ApiStore;
use crate::api::store::{ApiRequestAction, GetParameters, Gettable};
use crate::common::components::{select::SelectOption, Select};
use crate::features::fan_size::Store;

#[derive(Properties, PartialEq)]
pub struct FanSizePickerProps {
    #[prop_or(Rc::new(vec![]))]
    pub errs: Rc<Vec<String>>,
    pub option_predicate: Callback<FanSize<()>, bool>,
    pub no_selection_label: AttrValue,
    pub selection: Option<FanSize<()>>,
    pub on_select: Callback<Option<FanSize<()>>, ()>,
}

#[function_component]
pub fn FanSizePicker(
    FanSizePickerProps {
        errs,
        option_predicate,
        no_selection_label,
        selection,
        on_select,
    }: &FanSizePickerProps,
) -> Html {
    let gettable = Gettable::FanSizesIndex;

    let (_state, dispatch) = use_store::<ApiStore>();
    let fan_sizes: Rc<Vec<FanSize<()>>> = {
        use_selector_with_deps(
            move |state: &Store, option_predicate| {
                state
                    .fan_sizes
                    .values()
                    .filter(|val| option_predicate.emit((*val).clone()))
                    .cloned()
                    .collect::<Vec<_>>()
            },
            option_predicate.clone(),
        )
    };

    let request_status = use_selector_with_deps(
        |store: &ApiStore, dep_gettable| {
            store
                .get_status
                .get(dep_gettable)
                .cloned()
                .unwrap_or_default()
        },
        gettable.clone(),
    );

    use_effect_with_deps(
        move |dep_gettable| {
            dispatch.apply(ApiRequestAction::Get(
                GetParameters {
                    ignore_cache: false,
                },
                dep_gettable.clone(),
            ));

            || {}
        },
        gettable,
    );
    html! {
      <Select<FanSize<()>>
        errs={errs}
        no_selection_label={no_selection_label.clone()}
        selection={selection.clone()}
        on_select={on_select}
        request_status={request_status}
        selectables={fan_sizes}
      />
    }
}

impl SelectOption for FanSize<()> {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn label(&self) -> String {
        self.id.clone()
    }
}
