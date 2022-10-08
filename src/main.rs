use yew::prelude::*;

enum Msg { AddOne }
struct CtrComponent { ct:i64 }

impl Component for CtrComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self { Self {ct : 0} }

    fn update(&mut self, _ctx: &Context<Self>, msg:Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.ct += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class ="container">
                <p> {self.ct} </p>
                <button onclick={link.callback(|_|Msg::AddOne)}> {"+1"} </button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<CtrComponent>();
}
