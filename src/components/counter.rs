use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

#[function_component(Counter)]
pub fn counter(Props { title }: &Props) -> Html {
    let count_handle = use_state(|| 0);
    let count = *count_handle;

    let cmd_increment = {
        let count = count_handle.clone();
        Callback::from(move |_| {
            count_handle.set(*count + 1);
        })
    };

    html! {
        <>
            <h2>{title}</h2>
            <label>
                { "Count "} <p/>
                <input id="count" type="text" readonly=true value={count.to_string()}/>
                    <button type="button" onclick={cmd_increment} >{"Add"}</button>
            </label>
        </>
    }
}
