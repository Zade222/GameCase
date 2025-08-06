use std::path::PathBuf;

use cursive::{
    Cursive
};

use crate::cli_structs::{
    AppState, BuildStep
};

use crate::ui_elements::{
    file_and_directory_selector, show_build_screen
};

pub fn run_case_builder(
    siv: &mut Cursive
) {
    let current_step = siv
        .user_data::<AppState>()
        .unwrap().build_state.current_step.clone();

    match current_step {
        BuildStep::RomSelection => show_rom_selection_screen(siv),
    }

}

fn show_rom_selection_screen(siv: &mut Cursive) {
    let (staged_roms, start_path) = siv
        .with_user_data(|app_state: &mut AppState| {
            (
                app_state.build_state.staged_roms.clone(),
                app_state.config.default_browse_directory.clone(),
            )
        })
        .unwrap();

    let on_add = move |s: &mut Cursive| {
        let cb_sink = s.cb_sink().clone();

        let on_selection_callback = move |selected_paths: Option<Vec<PathBuf>>| {
            if let Some(paths) = selected_paths {
                cb_sink
                    .send(Box::new(move |siv: &mut Cursive| {
                        siv.with_user_data(|app_state: &mut AppState| {
                            app_state.build_state.staged_roms.extend(paths);
                        })
                        .unwrap();
                        
                        siv.pop_layer();
                        show_rom_selection_screen(siv);
                    }))
                    .unwrap();
            }
        };

        file_and_directory_selector(
            s.cb_sink().clone(), 
            start_path.clone(), 
            "Select a ROM file or a directory".to_string(), 
            false, 
            on_selection_callback
        );
    };

    let on_next = |s: &mut Cursive| {
        s.with_user_data(|app_state: &mut AppState| {
            // We just set the state back to where we want to be
            app_state.build_state.current_step = BuildStep::RomSelection;
        }).unwrap();
        
        s.pop_layer();
        run_case_builder(s);
    };

    show_build_screen(
        siv, 
        "Step 1: Add ROMs or an archive file".to_string(), 
        &staged_roms, 
        on_add, 
        on_next);
}