use yew::{
    html,
    Component,
    ComponentLink,
    Html,
    Renderable,
    ShouldRender,
};

struct Model {
    count: u64,
}

enum Msg {
    IncrementCounter,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            count: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::IncrementCounter => {
                self.count += 1;
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <button onclick=|_| Msg::IncrementCounter>{ "Click Me!" }</button>
                <p>{self.count}</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
