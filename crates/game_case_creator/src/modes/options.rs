use std::{
    path::{PathBuf},
    sync::{Arc, Mutex}
};

use cursive::Cursive;
use cursive::align::HAlign;
use cursive::views::{
    Dialog, SelectView,
};

use crate::cli_structs::{
    AppState, AppConfig
};

use crate::ui_elements::{
    file_and_directory_selector
};

use crate::{
    back_to_main_menu
};


pub fn run_options(
    siv: &mut Cursive,
    cfg: Arc<Mutex<AppConfig>>
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
                set_def_dir(s);
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
    cfg: Arc<Mutex<AppConfig>>
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
) {
    let cb_sink = siv.cb_sink().clone();

    let app_state = siv.user_data::<AppState>().unwrap().clone();
    let start_path = app_state.config.default_browse_directory.clone();

    let on_selection_callback = move |selected_paths: Option<Vec<PathBuf>>| {
        if let Some(paths) = selected_paths {
            let mut updated_config = app_state.config.clone();
            updated_config.default_browse_directory = paths[0].clone();
            
            confy::store(
                "boxer", 
                "boxer-config", 
                updated_config.clone()
            ).unwrap();

            let final_state = AppState {
                config: updated_config,
                build_state: app_state.build_state.clone(),
            };

            cb_sink.send(Box::new(move |s| {
                s.set_user_data(final_state);
                s.add_layer(Dialog::info(
                    "Default directory has been updated!"
                ));
            })).unwrap();
        }
    };

    file_and_directory_selector(
        siv.cb_sink().clone(), 
        start_path,
        "Select a new default directory.".to_string(),
        true,
        on_selection_callback
    );
}