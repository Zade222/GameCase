use std::{
    path::{PathBuf},
    sync::mpsc::{channel, Sender}
};

use cursive::{
    view::{Nameable, Resizable, Scrollable},
    views::{Dialog, DummyView, LinearLayout, SelectView, TextView},
    Cursive
};


use crate::storage_io::list_dir_items;

/// This is an internal helper function that creates the UI view.
/// It is called on the main UI thread via the CbSink.
fn selector_view(
    siv: &mut Cursive, 
    path: PathBuf, 
    tx: Sender<Option<PathBuf>>,
    task_text: String,
    select_only_dir: bool,
) {
    let mut select_view = SelectView::new().autojump();
    select_view.add_all(list_dir_items(&path));

    //When a user presses Enter on an item
    let tx_submit = tx.clone();
    let task_text_clone = task_text.clone();
    
    select_view.set_on_submit(move |s, selection: &PathBuf| {
        if selection.is_dir() {
            /*If it's a directory, pop the current layer and create a new
            selector view for the new directory.*/
            s.pop_layer();
            selector_view(
                s, 
                selection.clone(), 
                tx_submit.clone(),
                task_text_clone.clone(),
                select_only_dir
            );
        } else {
            //If it's a file, send the selection back and close the dialog.
            tx_submit.send(Some(selection.clone())).unwrap_or_default();
            s.pop_layer();
        }
    });

    let dialog = Dialog::around(
        select_view
            .with_name("selector")
            .scrollable()
            .fixed_size((50, 20)),
    )
    .title(path.to_string_lossy().into_owned())
    .button("Select", {
        let tx_button = tx.clone();
        let current_path = path.clone();
        move |s| {
            //Get the currently highlighted item from the SelectView.
            let selection_item = s
                .call_on_name("selector", |view: &mut SelectView<PathBuf>| {
                    view.selected_id().and_then(|id| {
                        view.get_item(id)
                            .map(|(label, value)| {
                                (label.to_string(), value.clone())
                        })
                    })
                })
                .flatten();

            if let Some((label, selected_path)) = selection_item {
                let final_path = if label == "../" {
                    //If user selects "../", use the current directory's path.
                    current_path.clone()
                } else {
                    //Otherwise, use the path of the highlighted item.
                    selected_path
                };

                if select_only_dir && !final_path.is_dir() {
                    s.add_layer(Dialog::info(
                        "Invalid selection: Please select a directory.",
                    ));
                    return;
                }

                tx_button.send(Some(final_path)).unwrap_or_default();
                s.pop_layer();
            }
        }
    })
    .button("Cancel", move |s| {
        //Send None to indicate cancellation and close the dialog.
        tx.send(None).unwrap_or_default();
        s.pop_layer();
    });

    let layout = LinearLayout::vertical()
        .child(TextView::new(
            task_text.clone()).center()
        )
        .child(DummyView)
        .child(dialog)
        .child(DummyView)
        .child(
            TextView::new(
                "Navigate with arrow keys and press <Select> or <Enter>."
            )
            .center(),
        );

    siv.add_layer(Dialog::around(layout));
}

/// Prompts the user to select a file or directory.
///
/// This function shows a dialog on the UI thread and blocks the calling thread
/// until the user selects a path or cancels the dialog.
///
/// Returns `Some(PathBuf)` if a path was selected, or `None` on cancellation.
pub fn file_and_directory_selector(
    siv_sink: &cursive::CbSink,
    start_path: PathBuf,
    task_text: String,
    select_only_dir: bool,
) -> Option<PathBuf> {
    /*Create a Multi-Producer, Single-Consumer channel.
    tx (transmitter) sends the result from the UI thread.
    rx (receiver) blocks the current thread waiting for the result.*/
    let (tx, rx) = channel();

    /*Send a function to the UI thread. This function will be executed
    by the Cursive event loop.*/
    siv_sink.send(Box::new(move |s: &mut Cursive| {
        /*The closure we send owns the `start_path` and the `tx` side of the 
        channel.*/
        selector_view(
            s, 
            start_path, 
            tx, 
            task_text.clone(), 
            select_only_dir
        );
    }))
    .expect("Failed to send function to Cursive event loop.");

    /*Block the current thread and wait for a message from the UI thread.
    `rx.recv()` returns a Result, so we use `ok()` to get an Option,
    and `flatten()` to turn `Some(Some(path))` into `Some(path)`
    and `Some(None)` into `None`.*/
    rx.recv().ok().flatten()
}