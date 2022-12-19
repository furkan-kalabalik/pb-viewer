extern crate cursive_tree_view;
use std::io;

// External Dependencies ------------------------------------------------------
use cursive::direction::Orientation;
use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout, Panel, ResizedView, TextView};
use cursive::Cursive;

// Modules --------------------------------------------------------------------
use cursive_tree_view::{Placement, TreeView};
use serde_json::{Map, Value};

fn generate_tree(value: &Value, tree: &mut TreeView<String>, row: usize) {
    match value {
        Value::Null => {
            tree.insert_item(format!("{}", "null"), Placement::LastChild, row);
        },
        Value::Bool(boolean) => {
            tree.insert_item(format!("{}", boolean), Placement::LastChild, row);
        },
        Value::Number(num) => {
            tree.insert_item(format!("{}", num), Placement::LastChild, row);
        },
        Value::String(json_str) => {
            tree.insert_item(format!("{}", json_str), Placement::LastChild, row);
        },
        Value::Array(arr) => {
            for (index, val) in arr.iter().enumerate(){
                let in_row = tree.insert_item(format!("{}-{}", "Item", index), Placement::LastChild, row).unwrap();
                generate_tree(val, tree, in_row);
            }
        },
        Value::Object(obj) => {
            for (key, val) in obj.iter(){
                let in_row = tree.insert_item(format!("{}", key), Placement::LastChild, row).unwrap();
                generate_tree(val, tree, in_row);
            }
        },
    }
}

pub fn run_ui(value: &Value) -> Result<(), io::Error> {
    let mut siv = cursive::default();

    // Tree -------------------------------------------------------------------
    let mut tree = TreeView::new();

    generate_tree(value, &mut tree, 0);

    // Callbacks --------------------------------------------------------------
    tree.set_on_submit(|siv: &mut Cursive, row| {
        let value = siv.call_on_name("tree", move |tree: &mut TreeView<String>| {
            tree.borrow_item(row).unwrap().to_string()
        });

        siv.add_layer(
            Dialog::around(TextView::new(value.unwrap()))
                .title("Item submitted")
                .button("Close", |s| {
                    s.pop_layer();
                }),
        );

        set_status(siv, row, "Submitted");
    });

    tree.set_on_select(|siv: &mut Cursive, row| {
        set_status(siv, row, "Selected");
    });

    tree.set_on_collapse(|siv: &mut Cursive, row, collpased, _| {
        if collpased {
            set_status(siv, row, "Collpased");
        } else {
            set_status(siv, row, "Unfolded");
        }
    });

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);
    
    // UI ---------------------------------------------------------------------

    siv.add_layer(Dialog::around(tree.with_name("tree").scrollable().full_screen()).title("pb-viewer"));
    fn set_status(siv: &mut Cursive, row: usize, text: &str) {
        let value = siv.call_on_name("tree", move |tree: &mut TreeView<String>| {
            tree.borrow_item(row)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "".to_string())
        });

        siv.call_on_name("status", move |view: &mut TextView| {
            view.set_content(format!(
                "Last action: {} row #{} \"{}\"",
                text,
                row,
                value.unwrap()
            ));
        });
    }

    siv.run();

    Ok(())
}

