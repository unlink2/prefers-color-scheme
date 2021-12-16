use prefers_color_scheme::{prefers_dark, prefers_light};
use yew::prelude::*;

enum Msg {
    Query,
}

struct Model {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Query => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::Query)}>{ "Query" }</button>
                <p>{"Prefers Dark:"} { prefers_dark() }</p>
                <p>{"Prefers Light:"} { prefers_light() }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
