//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  07no24                                             //
//---------------------------------------------------------------//



use yew::prelude::*;



#[function_component(NavbarComponent)]
pub fn navbar() -> Html {

    html! {

        <nav class="navbar navbar-expand-lg navbar-light bg-transparent fixed-top">
            <div class="container-fluid">
                <a class="navbar-brand" href="https://sfmolina.github.io/">
                    {"@sfmolina"}
                </a>
            </div>
        </nav>

    }

}
