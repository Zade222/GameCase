use std::{
    path::{PathBuf},
    sync::mpsc::{channel, Sender}
};

use cursive::{
    view::{Nameable, Resizable, Scrollable},
    views::{Dialog, DummyView, LinearLayout, SelectView, TextView},
    {CbSink, Cursive}
};

use crate::back_to_main_menu;

use crate::storage_io::list_dir_items;

/// The internal UI-building function for file_and_directory_selector
fn selector_view(
    siv: &mut Cursive, 
    path: PathBuf, 
    tx: Sender<Option<PathBuf>>,
    task_text: String,
    select_only_dir: bool,
) {
    let mut select_view = SelectView::<PathBuf>::new().autojump();

    
    select_view.add_all(list_dir_items(&path));

    let tx_for_submit = tx.clone();
    let task_text_for_submit = task_text.clone();

    select_view.set_on_submit(move |s, selection: &PathBuf| {
        if selection.is_dir() {
            s.pop_layer();
            selector_view(
                s,
                selection.clone(),
                tx_for_submit.clone(), 
                task_text_for_submit.clone(), 
                select_only_dir,
            );
        } else if !select_only_dir {
            //If it's a file and we are not in select_only_dir mode, send the selection.
            tx_for_submit.send(Some(selection.clone())).unwrap_or_default();
            s.pop_layer();
        }
    });

    //Clone the sender for the "Select" button's closure.
    let tx_for_select = tx.clone();

    let dialog = Dialog::around(
        select_view
            .with_name("selector")
            .scrollable()
            .fixed_size((50, 20)),
    )
    .title(path.to_string_lossy().into_owned())
    .button("Select", move |s| {
        let selection = s.call_on_name("selector", |v: &mut SelectView<PathBuf>| {
            // We get the selected value to handle both files and directories.
            v.selection()
        });

        if let Some(Some(selection)) = selection {
            let final_path = selection.clone();

            if select_only_dir && !final_path.is_dir() {
                s.add_layer(Dialog::info("Invalid selection: Please select a directory."));
                return;
            }

            tx_for_select.send(Some(final_path.to_path_buf())).unwrap_or_default();
            s.pop_layer();
        }
    })
    .button("Cancel", move |s| {
        // The original `tx` can be moved into the final closure without cloning.
        tx.send(None).unwrap_or_default();
        s.pop_layer();
    });

    let layout = LinearLayout::vertical()
        .child(TextView::new(&task_text).center())
        .child(DummyView)
        .child(dialog)
        .child(DummyView)
        .child(TextView::new("Navigate with arrow keys and press <Select> or <Enter>.").center());

    siv.add_layer(Dialog::around(layout));
}

/// Prompts the user to select a file or directory using a non-blocking UI.
///
/// Spawns a worker thread to await user input from the UI, then executes
/// the `on_selection` callback with the result.
pub fn file_and_directory_selector<F>(
    cb_sink: CbSink,
    start_path: PathBuf,
    task_text: String,
    select_only_dir: bool,
    on_selection: F
) where
    F: FnOnce(Option<PathBuf>) + Send + 'static,
{
    std::thread::spawn(move || {
        let (tx, rx) = channel();
        
        // Show the selector UI on the main thread
        cb_sink.send(Box::new(move |siv| {
            selector_view(siv, start_path, tx, task_text, select_only_dir);
        })).expect("Failed to send selector view to UI thread.");

        // Block this background thread until we get a result from the UI
        if let Ok(selected_path) = rx.recv() {
            // We got a result, now call the final callback
            on_selection(selected_path);
        }
    });
}

pub fn show_build_screen<F, N>(
    siv: &mut Cursive,
    builder_text: String,
    files_to_show: &[PathBuf],
    on_add: F,
    on_next: N,
) where
    F: Fn(&mut Cursive) + Send + Sync + 'static, 
    N: Fn(&mut Cursive) + Send + Sync + 'static,
{
    let mut select_view = SelectView::new().autojump();

    if files_to_show.is_empty() {
        select_view.add_item("[No files selected]", PathBuf::new());
        select_view.set_enabled(false);
    } else {
        for file_path in files_to_show {
            let file_name = file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            select_view.add_item(file_name, file_path.clone());
        }
    }

    let layout = LinearLayout::vertical()
        .child(TextView::new(builder_text).center())
        .child(DummyView)
        .child(select_view.scrollable().with_name("file_list"))
        .child(DummyView)
        .child(TextView::new("Navigate with arrow keys and press <Enter>.").center());

    let mut dialog = Dialog::around(layout)
        .title("Build a GameCase")
        .button("Add File/Directory", on_add);

    if !files_to_show.is_empty() {
        dialog.add_button("Next", on_next);
    }

    dialog.add_button("Cancel", |s| {
        back_to_main_menu(s);
    });

    siv.add_layer(dialog);
}