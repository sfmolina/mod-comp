//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  08no24                                             //
//---------------------------------------------------------------//



use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::perceptron_simple::PerceptronSimpleComponent;



#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/simple-perceptron")]
    PerceptronSimple,
}


pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Root => html! { <Redirect<Route> to={Route::PerceptronSimple}/> },
        Route::PerceptronSimple => html! { <PerceptronSimpleComponent /> },
    }
}
