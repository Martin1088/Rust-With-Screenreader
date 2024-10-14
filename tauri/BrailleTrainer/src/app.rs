use gloo_timers::callback::Interval;
use rand::Rng;
use yew::prelude::*;
use web_sys::HtmlElement;

#[function_component(App)]
pub fn app() -> Html {
    let dynamic_content = use_state(|| String::from("Initial content"));
    let dynamic_content_ref = use_node_ref();

    {
        let dynamic_content = dynamic_content.clone();
        let dynamic_content_ref = dynamic_content_ref.clone();
        use_effect_with((), move |_| {
            let dynamic_content = dynamic_content.clone();
            let dynamic_content_ref = dynamic_content_ref.clone();
            // Interval that updates content every 10 seconds
            let interval = Interval::new(10_000, move || {
                let mut rng = rand::thread_rng();
                let random_char = rng.gen_range(b'a'..=b'z') as char;
                let new_content = format!("{}{}", *dynamic_content, random_char);

                // Update the dynamic content
                dynamic_content.set(new_content.clone());

                // Focus the element programmatically to ensure the braille display reads it
                if let Some(element) = dynamic_content_ref.cast::<HtmlElement>() {
                    element.set_inner_text(&new_content);
                    element.focus().unwrap();
                }
            });

            move || drop(interval)
        });
    }

    html! {
        <main class="container">
            <p
                role="status"
                aria-live="assertive"
                aria-atomic="true"
                ref={dynamic_content_ref}
                tabindex="-1"
                id="dynamic-content">
                { &*dynamic_content }
            </p>
        </main>
    }
}