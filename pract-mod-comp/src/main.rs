//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  07no24                                             //
//---------------------------------------------------------------//



mod app;
mod components;
mod router;
mod utils;
mod data;



use app::App;



fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
