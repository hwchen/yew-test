use yew::{
    html,
    Component,
    ComponentLink,
    Html,
    IKeyboardEvent,
    Renderable,
    ShouldRender,
};

struct Model {
    count: u64,
    input_count: String,
    says: String,
}

enum Msg {
    IncrementCounter,
    GetInput(String),
    SetCounter,
    Pass,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            count: 0,
            input_count: "".to_owned(),
            says: "".to_owned(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::IncrementCounter => {
                if self.count < 10 {
                    self.count += 1;
                } else {
                    self.says = "can't increment more than 10".to_owned();
                }
                true
            }
            Msg::GetInput(n_str) => {
                self.input_count = n_str;
                false
            }
            Msg::SetCounter => {
                match self.input_count.parse() {
                    Ok(n) => {
                        if n <= 10 {
                            self.count = n;
                            self.input_count = "".to_owned();
                        } else {
                            self.says = "can't set more than 10".to_owned();
                        }
                    },
                    Err(_) => {
                        self.says = "Please enter a number".to_owned();
                    }
                }
                true
            }
            Msg::Pass => false,
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <button onclick=|_| Msg::IncrementCounter>{ "Click Me!" }</button>
                <p>{self.count}</p>
                <div>
                    <p>{ "Enter a number below to set counter" }</p>
                    <input
                        value=&self.input_count
                        oninput=|e| Msg::GetInput(e.value)
                        onkeyup=|e| if e.key() == "Enter" { Msg::SetCounter } else { Msg::Pass }
                    />
                </div>
                <p>{&self.says}</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
