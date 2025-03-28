mod reqwasm_mrbs;
use yew::prelude::*;


use self::reqwasm_mrbs::{call_mrbs_test, EntryRelatedData};

#[function_component]
pub fn App() -> Html {
    call_mrbs_test();
    let counter = use_state(|| 0);
    let onclick = {
        let counter =counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    html!(
        <>
            <h1> { "Raumplaner Demo" } </h1>
                <p>{ "Test System" }</p>
                <p>{ data} </p>
            <div>
            <button {onclick}>{ "+1" } </button>
            <p>{*counter}</p>
            </div>


        </>

    )
}
