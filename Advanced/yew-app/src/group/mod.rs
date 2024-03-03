mod reqwasm_mrbs;
use yew::prelude::*;

//use crate::group::request_mrbs::{mrbs_test, EntryRelatedData};

#[function_component]
pub fn App() -> Html {
//    let data: Vec<EntryRelatedData> = mrbs_test();
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
            <div>
            <button {onclick}>{ "+1" } </button>
            <p>{*counter}</p>
            </div>


        </>

    )
}
