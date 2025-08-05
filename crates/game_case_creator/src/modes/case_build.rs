use std::sync::{Arc, Mutex};

use cursive::{
    views::Dialog,
    Cursive
};

use crate::cli_structs::{
    BoxerConfig
};

use crate::utilites::{
     file_and_directory_selector
};

pub fn run_case_build(
    siv: &mut Cursive,
    cfg: Arc<Mutex<BoxerConfig>>
) {
    let cb_sink = siv.cb_sink().clone();
    let cfg_clone = cfg.clone();

    std::thread::spawn(move || {
        //Pop the "Loading..." dialog before showing the selector
        cb_sink.send(Box::new(|s| { s.pop_layer(); })).unwrap();

        let start_path = cfg_clone
            .lock()
            .unwrap().default_browse_directory
            .clone();

        if let Some(selected_path) = file_and_directory_selector(
            &cb_sink, 
            start_path,
            "Select a ROM or folder of ROMs.".to_string(),
            false
        ) {
                /*After getting the path, send another closure to the UI thread
                to continue the work, for example by showing another dialog.*/
                cb_sink.send(Box::new(move |s| {
                    s.add_layer(
                        Dialog::info(format!("You selected: \
                            {}\n\n(Next, you would implement the case \
                            building logic here.)", 
                            selected_path.to_string_lossy()))
                    );
                })).unwrap();
        } else {
            // The user cancelled.
            cb_sink.send(Box::new(|s| {
                s.add_layer(Dialog::info("File selection cancelled."));
            })).unwrap();
        }
    });
}