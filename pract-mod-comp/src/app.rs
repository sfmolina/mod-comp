//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  26oc24                                             //
//---------------------------------------------------------------//



use yew::prelude::*;
use yew_router::prelude::*;
use crate::{
    router::{switch, Route},
    components::nav_bar::NavbarComponent
};



#[function_component(App)]
pub fn app() -> Html {


    html! {
        <HashRouter>
            <main>
                <NavbarComponent />
                <Switch<Route> render={switch} />
            </main>
        </HashRouter>
    }
}
