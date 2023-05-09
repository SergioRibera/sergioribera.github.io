use once_cell::sync::Lazy;
use yew::prelude::*;

pub const APPLICATION_NAME: Lazy<String> =
    Lazy::new(|| obfstr::obfstr!(env!("APPLICATION_NAME")).to_string());

pub struct App;

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        document.set_title(APPLICATION_NAME.as_str());
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(<h1>{"Hola Mundo!"}</h1>)
    }
}
