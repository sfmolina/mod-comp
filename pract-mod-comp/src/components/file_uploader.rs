//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  20no24                                             //
//---------------------------------------------------------------//



use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{File, FileReader};
use yew::prelude::*;



#[derive(Properties, PartialEq)]
pub struct FileUploadProps {
    pub on_file_upload: Callback<Vec<Vec<f64>>>,
}


#[function_component(FileUpload)]
pub fn file_upload(props: &FileUploadProps) -> Html {
    let file_ref = use_node_ref();
    
    let onchange = {
        let file_ref = file_ref.clone();
        let on_file_upload = props.on_file_upload.clone();
        Callback::from(move |_| {
            if let Some(file_input) = file_ref.cast::<web_sys::HtmlInputElement>() {
                if let Some(files) = file_input.files() {
                    if let Some(file) = files.get(0) {
                        process_file(file, on_file_upload.clone());
                    }
                }
            }
        })
    };

    html! {
        <input class="custom-file-input" type="file" ref={file_ref} {onchange} />
    }
}


fn process_file(file: File, callback: Callback<Vec<Vec<f64>>>) {
    let file_reader = FileReader::new().unwrap();
    let file_reader_clone = file_reader.clone();

    let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
        if let Ok(result) = file_reader_clone.result() {
            if let Some(text) = result.as_string() {
                let data = parse_file_content(&text);
                callback.emit(data);
            }
        }
    }) as Box<dyn FnMut(_)>);

    file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
    file_reader.read_as_text(&file).expect("Failed to read file");
    onload.forget();
}


fn parse_file_content(content: &str) -> Vec<Vec<f64>> {
    content
        .lines()
        .filter_map(|line| {
            let numbers: Vec<f64> = line
                .split(',')
                .filter_map(|num| num.trim().parse().ok())
                .collect();
            if numbers.is_empty() {
                None
            } else {
                Some(numbers)
            }
        })
        .collect()
}
