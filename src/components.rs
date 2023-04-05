use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

#[function_component(Counter)]
pub fn counter(Props { title }: &Props) -> Html {
    let count = use_state(|| 0);
    let cmd_increment = {
        let count = count.clone();
        Callback::from(move |_| {
            count.set(*count + 1);
        })
    };

    html! {
        <>
            <h2>{title}</h2>
            <label>
                { "Count "} <p/>
                <input id="count" type="text" value={(*count).to_string()}/>
                    <button type="button" onclick={cmd_increment} >{"Add"}</button>
            </label>
        </>
    }
}
