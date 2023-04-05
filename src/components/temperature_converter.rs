// use log::info;
use std::num::ParseFloatError;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

#[derive(PartialEq, Debug)]
struct Temperature {
    celsius: f32,
    fahrenheit: f32,
    invalid_change: Option<TemperatureInput>,
}

/// Immutable state container for temp in f and c, and last input
impl Default for Temperature {
    fn default() -> Self {
        const CELSIUS: f32 = 0.0;
        Self {
            celsius: CELSIUS,
            fahrenheit: Self::f_to_c(CELSIUS),
            invalid_change: None,
        }
    }
}

impl Clone for Temperature {
    fn clone(&self) -> Self {
        Self {
            celsius: self.celsius,
            fahrenheit: self.fahrenheit,
            invalid_change: self.invalid_change.clone(),
        }
    }
}

impl Temperature {
    fn with_change_candidate(self, value: TemperatureInput) -> Self {
        let mut celsius: f32 = self.celsius;
        let mut fahrenheit: f32 = self.fahrenheit;
        let mut invalid_change = None;
        match value.to_float() {
            Ok(temperature_value) => match value {
                TemperatureInput::Celsius(_) => {
                    celsius = temperature_value;
                    fahrenheit = Self::c_to_f(temperature_value);
                }
                TemperatureInput::Fahrenheit(_) => {
                    fahrenheit = temperature_value;
                    celsius = Self::f_to_c(temperature_value);
                }
            },
            Err(_) => {
                invalid_change = Some(value);
            }
        }

        Self {
            celsius,
            fahrenheit,
            invalid_change,
        }
    }

    fn last_c(&self) -> (bool, String) {
        match self.invalid_change {
            Some(ref change) => {
                if let TemperatureInput::Celsius(value) = change {
                    (false, value.to_string())
                } else {
                    (true, self.celsius.to_string())
                }
            }
            None => (true, self.celsius.to_string()),
        }
    }

    fn last_f(&self) -> (bool, String) {
        match self.invalid_change {
            Some(ref change) => {
                if let TemperatureInput::Fahrenheit(value) = change {
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
enum TemperatureInput {
    Celsius(String),
    Fahrenheit(String),
}

impl Clone for TemperatureInput {
    fn clone(&self) -> Self {
        match self {
            Self::Celsius(value) => Self::Celsius(value.clone()),
            Self::Fahrenheit(value) => Self::Fahrenheit(value.clone()),
        }
    }
}

#[derive(Debug)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl TemperatureUnit {
    fn to_input(&self, value: String) -> TemperatureInput {
        match self {
            TemperatureUnit::Celsius => TemperatureInput::Celsius(value),
            TemperatureUnit::Fahrenheit => TemperatureInput::Fahrenheit(value),
        }
    }
}

impl TemperatureInput {
    fn to_float(&self) -> Result<f32, ParseFloatError> {
        match self {
            TemperatureInput::Celsius(ref val) => val.parse(),
            TemperatureInput::Fahrenheit(ref val) => val.parse(),
        }
    }
}

#[function_component(TemperatureConverter)]
pub fn temperature_converter(Props { title }: &Props) -> Html {
    let temperature_state = use_state(Temperature::default);

    let change_temp_callback = |unit: TemperatureUnit| {
        let temperature_state = temperature_state.clone();
        Callback::from(move |event: InputEvent| {
            let input_element = event.target_unchecked_into::<HtmlInputElement>();
            let input_value = input_element.value();
            let input_temperature = unit.to_input(input_value);
            let temperature = (*temperature_state)
                .clone()
                .with_change_candidate(input_temperature);
            temperature_state.set(temperature);
        })
    };

    let f = temperature_state.last_f().1;
    let f_is_valid = temperature_state.last_f().0;
    let c = temperature_state.last_c().1;
    let c_is_valid = temperature_state.last_c().0;

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
