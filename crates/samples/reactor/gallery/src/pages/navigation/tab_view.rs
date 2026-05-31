use crate::controls::*;
use windows_reactor::*;

pub fn tab_view_page(_: &(), cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(0_i32);
    let (tab_ids, set_tab_ids) = cx.use_state(vec![1_i32, 2, 3]);
    let (next_id, set_next_id) = cx.use_state(4_i32);
    let (add_btn_count, set_add_btn_count) = cx.use_state(2_i32);

    let dynamic_tabs: Vec<TabItem> = tab_ids
        .iter()
        .map(|&i| {
            TabItem::new(
                format!("Tab {i}"),
                text_block(format!("Content of Tab {i}")),
            )
            .with_key(format!("{i}"))
        })
        .collect();

    let tab_count = tab_ids.len() as i32;

    page_content(
        "TabView",
        "A control that displays closable, rearrangeable tabs.",
        vec![
            sample_card(
                "Basic TabView",
                TabView::new([
                    TabItem::new("Home", text_block("Home content")),
                    TabItem::new("Document", text_block("Document content")),
                    TabItem::new("Settings", text_block("Settings content")),
                ])
                .selected_index(selected)
                .on_selection_changed({
                    let set_selected = set_selected.clone();
                    move |i: i32| set_selected.call(i)
                })
                .height(200.0),
                r#"TabView::new([
    TabItem::new("Home", content),
    TabItem::new("Document", content),
]).selected_index(idx).on_selection_changed(handler)"#,
            ),
            sample_card(
                "Dynamic Tabs",
                vstack((
                    TabView::new(dynamic_tabs)
                        .selected_index(selected.min(tab_count - 1))
                        .on_selection_changed({
                            let set_selected = set_selected;
                            move |i: i32| set_selected.call(i)
                        })
                        .on_tab_close_requested({
                            let tab_ids = tab_ids.clone();
                            let set_tab_ids = set_tab_ids.clone();
                            move |key: String| {
                                if let Ok(id) = key.parse::<i32>() {
                                    let remaining: Vec<i32> =
                                        tab_ids.iter().copied().filter(|&x| x != id).collect();
                                    if !remaining.is_empty() {
                                        set_tab_ids.call(remaining);
                                    }
                                }
                            }
                        })
                        .height(180.0),
                    hstack((
                        button("Add Tab").on_click({
                            let tab_ids = tab_ids.clone();
                            let set_tab_ids = set_tab_ids.clone();
                            let set_next_id = set_next_id.clone();
                            move || {
                                let mut ids = tab_ids.clone();
                                ids.push(next_id);
                                set_tab_ids.call(ids);
                                set_next_id.call(next_id + 1);
                            }
                        }),
                        button("Remove Tab").enabled(tab_count > 1).on_click({
                            let tab_ids = tab_ids.clone();
                            move || {
                                let mut ids = tab_ids.clone();
                                ids.pop();
                                set_tab_ids.call(ids);
                            }
                        }),
                    ))
                    .spacing(8.0),
                ))
                .spacing(8.0),
                r#"TabItem::new("Tab", content).with_key("id")
TabView::new(tabs)
    .on_tab_close_requested(|key| {
        set_ids.call(ids.filter(|x| x != key))
    })"#,
            ),
            sample_card(
                "Add Tab Button (+)",
                TabView::new(
                    (1..=add_btn_count)
                        .map(|i| TabItem::new(format!("Tab {i}"), text_block(format!("Content {i}"))))
                        .collect::<Vec<_>>(),
                )
                .on_add_tab_button_click({
                    let set_add_btn_count = set_add_btn_count.clone();
                    move || set_add_btn_count.call(add_btn_count + 1)
                })
                .height(180.0),
                r#"TabView::new(tabs)
    .on_add_tab_button_click(|| set_count.call(count + 1))"#,
            ),
            sample_card(
                "Non-closable Tabs",
                TabView::new([
                    TabItem::new("Fixed A", text_block("Always present")).closable(false),
                    TabItem::new("Fixed B", text_block("Cannot close")).closable(false),
                ])
                .height(150.0),
                r#"TabItem::new("Tab", content).closable(false)"#,
            ),
        ],
    )
}
