// use log::info;
use std::num::ParseFloatError;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

/// Immutable state container for temp in f and c, and last input
#[derive(PartialEq, Debug)]
struct TemperatureState {
    celsius: f32,
    fahrenheit: f32,
    failed_edit: Option<TemperatureEdit>,
}

impl Default for TemperatureState {
    fn default() -> Self {
        const CELSIUS: f32 = 0.0;
        Self {
            celsius: CELSIUS,
            fahrenheit: Self::f_to_c(CELSIUS),
            failed_edit: None,
        }
    }
}

impl Clone for TemperatureState {
    fn clone(&self) -> Self {
        Self {
            celsius: self.celsius,
            fahrenheit: self.fahrenheit,
            failed_edit: self.failed_edit.clone(),
        }
    }
}

impl TemperatureState {
    fn with_edit(mut self, edit: TemperatureEdit) -> Self {
        match edit.value() {
            Ok(temperature_value) => match edit {
                TemperatureEdit::Celsius(_) => {
                    self.celsius = temperature_value;
                    self.fahrenheit = Self::c_to_f(temperature_value);
                    self.failed_edit = None;
                }
                TemperatureEdit::Fahrenheit(_) => {
                    self.fahrenheit = temperature_value;
                    self.celsius = Self::f_to_c(temperature_value);
                    self.failed_edit = None;
                }
            },
            Err(_) => {
                self.failed_edit = Some(edit);
            }
        }

        self.clone()
    }

    /// Returns (false, "failed edit value") if c is currently being edited, and is invalid
    /// (true, "value") otherwise
    fn c_display(&self) -> (bool, String) {
        match self.failed_edit {
            Some(ref edit) => {
                if let TemperatureEdit::Celsius(value) = edit {
                    (false, value.to_string())
                } else {
                    (true, self.celsius.to_string())
                }
            }
            None => (true, self.celsius.to_string()),
        }
    }

    /// Returns (false, "failed edit value") if f is currently being edited, and is invalid
    /// (true, "value") otherwise
    fn f_display(&self) -> (bool, String) {
        match self.failed_edit {
            Some(ref edit) => {
                if let TemperatureEdit::Fahrenheit(value) = edit {
                    (false, value.to_string())
                } else {
                    (true, self.fahrenheit.to_string())
                }
            }
            None => (true, self.fahrenheit.to_string()),
        }
    }

    fn f_to_c(f: f32) -> f32 {
        (f - 32.0) * (5.0 / 9.0)
    }

    fn c_to_f(c: f32) -> f32 {
        c * (9.0 / 5.0) + 32.0
    }
}

#[derive(Debug, PartialEq)]
enum TemperatureEdit {
    Celsius(String),
    Fahrenheit(String),
}

impl Clone for TemperatureEdit {
    fn clone(&self) -> Self {
        match self {
            Self::Celsius(value) => Self::Celsius(value.clone()),
            Self::Fahrenheit(value) => Self::Fahrenheit(value.clone()),
        }
    }
}

impl TemperatureEdit {
    fn value(&self) -> Result<f32, ParseFloatError> {
        match self {
            TemperatureEdit::Celsius(ref val) => val.parse(),
            TemperatureEdit::Fahrenheit(ref val) => val.parse(),
        }
    }
}

#[derive(Debug)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl TemperatureUnit {
    fn to_input(&self, value: String) -> TemperatureEdit {
        match self {
            TemperatureUnit::Celsius => TemperatureEdit::Celsius(value),
            TemperatureUnit::Fahrenheit => TemperatureEdit::Fahrenheit(value),
        }
    }
}

#[function_component(TemperatureConverter)]
pub fn temperature_converter(Props { title }: &Props) -> Html {
    let temperature_state = use_state(TemperatureState::default);

    let change_temp_callback = |unit: TemperatureUnit| {
        let temperature_state = temperature_state.clone();
        Callback::from(move |event: InputEvent| {
            let input_element = event.target_unchecked_into::<HtmlInputElement>();
            let input_value = input_element.value();
            let input_temperature = unit.to_input(input_value);
            let temperature = (*temperature_state).clone().with_edit(input_temperature);
            temperature_state.set(temperature);
        })
    };

    let f = temperature_state.f_display().1;
    let f_is_valid = temperature_state.f_display().0;
    let c = temperature_state.c_display().1;
    let c_is_valid = temperature_state.c_display().0;

    html! {
        <>
            <h2>{title}</h2>
            <label>
                {"Celsius"}
                <input
                    id="temp_c"
                    style={if c_is_valid {"background-color: white "} else {"background-color: #ef9a9a"}}
                    type="text" value={c}
                    oninput={change_temp_callback(TemperatureUnit::Celsius)} />
            </label>
            <label>
                { "Fahrenheit"}
                <input
                    id="temp_f"
                    style={if f_is_valid {"background-color: white "} else {"background-color: #ef9a9a"}}
                    type="text" value={f}
                    oninput={change_temp_callback(TemperatureUnit::Fahrenheit)} />
            </label>
        </>
    }
}
