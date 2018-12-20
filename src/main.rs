#[macro_use]
extern crate yew;
use yew::prelude::*;

type Context = ();

struct Model {  }

enum Msg {
    DoIt,
}

impl Component<Context> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {  }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                // Update your model on events
                true
            }
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            // Render your model here
            <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>
        }
    }
}


fn main() {
    // Initialize the application
    yew::initialize();

    // Create a new app
    let app: App<_, Model> = App::new(());
    app.mount_to_body();

    // Run the application for development purposes
    yew::run_loop();
}
