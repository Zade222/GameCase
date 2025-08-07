use std::{
    path::{Path},
    sync::{Arc, Mutex},
};

use cursive::Cursive;
use cursive::align::HAlign;
use cursive::views::{Dialog, SelectView};

mod cli_error_handling;
use cli_error_handling::CliError;

mod cli_structs;
use cli_structs::{
    AppState, AppConfig
};

mod modes;
use modes::{
    run_options, run_case_builder
};

mod ui_elements;

mod storage_io;

fn main() -> Result<(), CliError>{
    //Load config on start
    let cfg: AppConfig = match confy::load("boxer", "boxer-config") {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            // You might want to use a default configuration in case of an error
            AppConfig::default() 
        }
    };

    //The Cursive root
    let mut siv = cursive::default();

    let initial_state = AppState {
        config: cfg.clone(),
        build_state: Default::default(),
        recent_dir: cfg.default_browse_directory.clone()
    };
    siv.set_user_data(initial_state);

    siv.set_window_title("Game Case Creator");
    
    let mut select = SelectView::new()
        //Center the text horizontally
        .h_align(HAlign::Center)
        //Use keyboard to jump to the pressed letters
        .autojump();

    let options = vec![
        "Create and build a GameCase",
        "Read GameCase File Info",
        "Options",
    ];

    select.add_all_str(options);

    select.set_on_submit(move |s, selection: &str| {
        match selection {
            "Create and build a GameCase" => {
                run_case_builder(s);
            }
            "Read GameCase File Info" => {

            }
            "Options" => {
                run_options(s, cfg.clone());
            }
            _ => {} //Should not happen but is required.
        }
    });
    
    siv.add_layer(
        Dialog::around(select)
            .title("Choose an option to get started:")
            .button("Quit", |s| s.quit()),
    );
    

    //Start the event loop
    siv.run();
    Ok(())
}

/// Pops all layers off the stack until only the root layer remains.
pub fn back_to_main_menu(siv: &mut Cursive) {
    let layer_count = siv.screen_mut().len();
    for _ in 1..layer_count {
        siv.pop_layer();
    }
}

pub fn update_rec_dir(siv: &mut Cursive, path: &Path) {
    siv.with_user_data(|app_state: &mut AppState| {
        app_state.recent_dir = path.to_path_buf();
    });
}