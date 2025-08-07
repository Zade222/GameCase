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
        .unwrap()
        .build_state
        .current_step
        .clone();

    match current_step {
        BuildStep::RomSelection => show_selection_screen(
            siv,
            current_step,
            "Step 1: Add ROMs or an archive file",
            "Select a ROM file or a directory".to_string(),
        ),
        BuildStep::ManualSelection => show_selection_screen(
            siv,
            current_step,
            "Step 2: Add manuals",
            "Select a manual file".to_string(),
        ),
    }

}

fn show_selection_screen(
    siv: &mut Cursive,
    step: BuildStep,
    title: &str,
    task_text: String,
) {
    let (staged_files, start_path) = siv
        .with_user_data(|app_state: &mut AppState| {
            let staged_files = match step {
                BuildStep::RomSelection => app_state.build_state.staged_roms.clone(),
                BuildStep::ManualSelection => app_state.build_state.staged_manuals.clone(),
            };
            (staged_files, app_state.config.default_browse_directory.clone())
        })
        .expect("Could not get AppState");
    let on_next_step = step.clone();

    let on_add = move |s: &mut Cursive| {
        let cb_sink = s.cb_sink().clone();
        let current_step_clone = step.clone();

        let on_selection_callback = move |selected_paths: Option<Vec<PathBuf>>| {
            if let Some(paths) = selected_paths {
                cb_sink
                    .send(Box::new(move |siv: &mut Cursive| {
                        siv.with_user_data(|app_state: &mut AppState| {
                            let staged_files_mut = match current_step_clone {
                                BuildStep::RomSelection => &mut app_state.build_state.staged_roms,
                                BuildStep::ManualSelection => &mut app_state.build_state.staged_manuals,
                            };
                            staged_files_mut.extend(paths);
                        })
                        .expect("Could not get AppState");

                        siv.pop_layer();
                        run_case_builder(siv);
                    }))
                    .expect("Could not send callback to UI thread");
            }
        };

        file_and_directory_selector(
            s.cb_sink().clone(),
            task_text.to_string(),
            false,
            on_selection_callback,
        );
    };

    let on_next = move |s: &mut Cursive| {
        s.with_user_data(|app_state: &mut AppState| {
            app_state.build_state.current_step = match on_next_step.clone() {
                BuildStep::RomSelection => BuildStep::ManualSelection,
                BuildStep::ManualSelection => BuildStep::ManualSelection,
            };
        })
        .expect("Could not get AppState");

        s.pop_layer();
        run_case_builder(s);
    };

    show_build_screen(
        siv,
        title.to_string(),
        &staged_files,
        on_add,
        on_next,
    );
}