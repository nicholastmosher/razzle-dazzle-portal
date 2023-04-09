use yew::prelude::*;

pub struct App {}

pub enum AppMsg {
    PlusOne,
}

impl Component for App {
    type Message = AppMsg;

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class="w-full h-full bg-blue-300">
                <form action="http://localhost:3000/upload" method="post" enctype="multipart/form-data">
                    <label>
                        {"Upload file:"}
                        <input type="file" name="file" multiple={true} />
                    </label>

                    <input type="submit" value="Upload files" />
                </form>
            </div>
        }
    }
}
