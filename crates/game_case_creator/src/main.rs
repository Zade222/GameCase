use std::{
    sync::{Arc, Mutex},
};

use cursive::Cursive;
use cursive::align::HAlign;
use cursive::views::{Dialog, SelectView};

mod cli_error_handling;
use cli_error_handling::CliError;

mod cli_structs;
use cli_structs::{
    BoxerConfig
};

mod modes;
use modes::{
    run_options, run_case_build
};

mod utilites;

mod storage_io;

fn main() -> Result<(), CliError>{
    //Load config on start
    let cfg: Arc<Mutex<BoxerConfig>> = Arc::new(
        Mutex::new(
            confy::load(
                "boxer", 
                "boxer-config"
            )?
        )
    );

    //The Cursive root
    let mut siv = cursive::default();

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
                run_case_build(s, cfg.clone());
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