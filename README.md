
# color-scheme

![](https://github.com/unlink2/prefers-color-scheme/actions/workflows/build.yml/badge.svg)
![](https://github.com/unlink2/prefers-color-scheme/actions/workflows/test.yml/badge.svg)

## Table of content

- [Installation](#Installation)
- [Usage](#Usage)
- [License](#License)
- [Contributing](#Contributing)
- [TODO](#TODO)

## Installation

## Usage

This is a very simple binding to the prefers-color-scheme media query.
The sample program in example can be run with `trunk serve`.

Usage:

```rust
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
```

### Running tests

Tests can be run using `wasm-pack test --headless --firefox`
or `wasm-pack test --headless --chrome`

## License

This program is distributed under the terms of the MIT License.

## Contributing

All contributions are welcome.
Both pull requests and issue reports are always appreciated.
Please make sure that all existing tests pass before submitting a pull request.

## TODO

