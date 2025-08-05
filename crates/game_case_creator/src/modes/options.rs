use std::sync::{Arc, Mutex};

use cursive::Cursive;
use cursive::align::HAlign;
use cursive::views::{
    Dialog, SelectView,
};

use crate::cli_structs::{
    BoxerConfig
};

use crate::utilites::{
    file_and_directory_selector
};

use crate::{
    back_to_main_menu
};


pub fn run_options(
    siv: &mut Cursive,
    cfg: Arc<Mutex<BoxerConfig>>
) {
    
    
    let options = vec![
        "Set Preferred Region",
        "Set Default Browse Directory",
        "Option 9001",
    ];
    
    let mut select = SelectView::new()
        //Center the text horizontally
        .h_align(HAlign::Center)
        //Use keyboard to jump to the pressed letters
        .autojump();

    select.add_all_str(options);

    select.set_on_submit(move |s, selection: &str| {
        match selection {
            "Set Preferred Region" => {
                set_region(s, cfg.clone());
            }
            "Set Default Browse Directory" =>{
                set_def_dir(s, cfg.clone());
            }
            _ => {} //Should not happen but is required.
        }
    });

    siv.add_screen();

    siv.add_layer(
        Dialog::around(select)
            .title("Choose an option to change.")
            .button("Go Back", |s| {
                s.pop_layer();
            }),
    );
}

fn set_region(
    siv: &mut Cursive,
    cfg: Arc<Mutex<BoxerConfig>>
) {
    let regions = vec![
        "Europe",
        "Japan",
        "United States",
        "World",
    ];

    let mut region = SelectView::new()
        //Center the text horizontally
        .h_align(HAlign::Center)
        //Use keyboard to jump to the pressed letters
        .autojump();

    region.add_all_str(regions);

    region.set_on_submit(move |s, selection: &str| {
        let mut config_data = cfg.lock().unwrap();

        match selection {
            "Europe" => {
                config_data.region = "Europe".to_string()
            }
            "Japan" => {
                config_data.region = "Japan".to_string()
            }
            "United States" => {
                config_data.region = "United States".to_string()
            }
            "World" => {
                config_data.region = "World".to_string()
            }
            _ => {} //Should not happen but is required.
        }
        
        confy::store(
            "boxer", 
            "boxer-config", 
            &*config_data
        ).unwrap();
    
        s.pop_layer();
    });

    siv.add_layer(
        Dialog::around(region)
            .title("Choose a region.")
            .button("Go Back", |s| {
                s.pop_layer();
            })
            .button("Main Menu", |s| {
                back_to_main_menu(s);
            }),
    );
}

fn set_def_dir(
    siv: &mut Cursive,
    cfg: Arc<Mutex<BoxerConfig>>
) {
    let cb_sink = siv.cb_sink().clone(); //Callback sink
    let cfg_clone = cfg.clone();

    std::thread::spawn(move || {
        let start_path = cfg_clone
            .lock()
            .unwrap().default_browse_directory
            .clone();

        if let Some(selected_path) = file_and_directory_selector(
            &cb_sink, 
            start_path,
            "Select a new default directory.".to_string(),
            false)
        {
            let mut config_data = cfg_clone.lock().unwrap();

            let path_to_set = if selected_path.is_dir() {
                selected_path
            } else {
                selected_path
                    .parent()
                    .unwrap_or(&selected_path)
                    .to_path_buf()
            };

            config_data.default_browse_directory = path_to_set;
            confy::store(
                "boxer", 
                "boxer-config", 
                &*config_data
            ).unwrap();

            cb_sink.send(Box::new(|s| {
                s.add_layer(Dialog::info("Default directory updated!"));
            })).unwrap();
        }
    });
}