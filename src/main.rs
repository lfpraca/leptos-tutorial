use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0u8);
    let double_count = move || count.with(|value| value * 2);

    #[rustfmt::skip]
    view! { cx,
	<button
	    on:click= move |_| {
		set_count.update(|value| *value += 1);
	    }
	    class=("button-20", move || count.with(|value| value % 2 == 1))
	>
	    "Click me: "
	    { move || count.get() }
	</button>
	<progress
	    max="50"
	    value= double_count
	/>
	<p>
	    "Double Count: "
	    { double_count }
	</p>
    }
}
