use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_callback, use_effect_with_deps, use_node_ref, use_state,
    Callback, Html, Properties, UseStateHandle,
};

#[derive(Properties, PartialEq)]
pub struct DeterminationsPasteTextAreaProps<
    const EXTRACTED_COLS: usize,
    const EXTRACTED_ROWS: usize,
> {
    pub on_extracted: Callback<Vec<[String; EXTRACTED_COLS]>>,
    pub expected_headers: Vec<&'static str>,
    pub cols_to_extract: [usize; EXTRACTED_COLS],
    pub expected_row_length: usize,
}

#[function_component]
pub fn DeterminationsPasteTextArea<const EXTRACTED_COLS: usize, const EXTRACTED_ROWS: usize>(
    DeterminationsPasteTextAreaProps {
        on_extracted,
        expected_headers,
        cols_to_extract,
        expected_row_length,
    }: &DeterminationsPasteTextAreaProps<EXTRACTED_COLS, EXTRACTED_ROWS>,
) -> Html {
    let total_rows = EXTRACTED_ROWS + expected_headers.len() + 1;
    let warnings: UseStateHandle<Vec<String>> = use_state(Vec::new);
    let errors: UseStateHandle<Vec<String>> = use_state(Vec::new);

    let determination_paste_ref = use_node_ref();
    let on_determination_paste = {
        let determination_paste_ref = determination_paste_ref.clone();
        let expected_headers = expected_headers.clone();
        let on_extracted = on_extracted.clone();
        let cols_to_extract = *cols_to_extract;
        let expected_row_length = *expected_row_length;
        let warnings_setter = warnings.setter();
        let errors_setter = errors.setter();
        use_callback(
            move |_evt, _dep| {
                let mut errors: Vec<String> = vec![];
                let mut warnings: Vec<String> = vec![];

                let input = determination_paste_ref
                    .cast::<HtmlInputElement>()
                    .expect("input_ref not attached to input element");

                let input_val: String = input.value();
                let mut text_rows = input_val.split('\n');

                warnings.extend(
                    expected_headers
                        .iter()
                        .enumerate()
                        .filter_map(|(i, header)| {
                            if text_rows.next() != Some(header) {
                                Some(format!("Header row #{} doesn't match expected", i + 1))
                            } else {
                                None
                            }
                        }),
                );

                let grid: Vec<[String; EXTRACTED_COLS]> = text_rows
                    .enumerate()
                    .map(|(i, row_str)| {
                        let split_row = row_str.split_whitespace().collect::<Vec<_>>();
                        if split_row.len() != expected_row_length {
                            warnings
                                .push(format!("Determination Row #{}'s length isn't right", i + 1));
                            cols_to_extract.map(|_col_id| "".to_string())
                        } else {
                            cols_to_extract.map(|col_id| {
                                split_row
                                    .get(col_id)
                                    .map_or("".to_string(), |s| s.to_string())
                            })
                        }
                    })
                    .collect::<Vec<_>>();
                if grid.len() != EXTRACTED_ROWS {
                    errors.push("Paste doesn't have the correct number of rows".to_string());
                }
                if errors.is_empty() {
                    on_extracted.emit(grid);
                }
                errors_setter.set(errors);
                warnings_setter.set(warnings);
            },
            (),
        )
    };
    {
        let determination_paste_ref = determination_paste_ref.clone();
        use_effect_with_deps(
            move |(errors, warnings)| {
                if let Some(input) = determination_paste_ref.cast::<HtmlInputElement>() {
                    if !errors.is_empty() {
                        input.set_custom_validity(&errors.join("\n"));
                    } else if !warnings.is_empty() {
                        input.set_custom_validity(&warnings.join("\n"));
                    } else {
                        input.set_custom_validity("");
                    }
                    input.report_validity();
                }
            },
            (errors, warnings),
        );
    }

    html! {
        <textarea
            ref={determination_paste_ref}
            rows={total_rows.to_string()}
            cols={"50"}
            // onpaste={on_determination_paste.clone()}
            onchange={on_determination_paste}
        >
        </textarea>
    }
}
