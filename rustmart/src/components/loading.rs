use yew::prelude::*;

pub struct Loading {}

impl Component for Loading {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }
    fn view(&self) -> Html {
        html! {
            <div class="loading_spinner_container">
                <div class="loading_spinner"></div>
                <div class="loading_spinner_text">{"Loading ..."}</div>
            </div>
        }
    }
}
