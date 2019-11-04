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
    says: String,
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
            says: "".to_owned(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::IncrementCounter => {
                if self.count < 10 {
                    self.count += 1;
                } else {
                    self.says = "can't do more than 10".to_owned();
                }
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
                <p>{&self.says}</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
