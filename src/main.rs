use yew::prelude::*;

enum Msg {
    AddOne,
}

struct App {
    value: u64,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={"container"}>
                <p>{"Counter"}</p>
                <p>{ self.value }</p>
                <button onclick={ctx.link().callback(|_| Msg::AddOne)}>{ "+1" }</button>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}