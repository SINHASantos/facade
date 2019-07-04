use crate::widgets::{View, Widget, WidgetModel};
use yew::html;

pub type Button = WidgetModel<Model>;

pub struct Model {}

impl Default for Model {
    fn default() -> Self {
        Self {}
    }
}

impl Widget for Model {
    type Properties = ();

    fn main_view(&self) -> View<Self> {
        html! {
            <button>{ "Button!" }</button>
        }
    }
}
