use crate::ui::window::notebook::playlist::item::{
    TrackItem, PROP_DURATION, PROP_ICON_NAME, PROP_QUEUE_POS,
};
use gtk::glib;
use gtk::prelude::*;
use gtk::{ColumnViewColumn, Inscription, SignalListItemFactory};

/// Binds `property` of the row's `TrackItem` to `target_property` of the cell
/// widget. The expression follows `ListItem:item`, so it survives row recycling
/// and picks up property notifications without rebinding the row.
fn bind_item_property(
    list_item: &gtk::ListItem,
    property: &str,
    widget: &impl IsA<glib::Object>,
    target_property: &str,
) {
    list_item
        .property_expression("item")
        .chain_property::<TrackItem>(property)
        .bind(widget, target_property, gtk::Widget::NONE);
}

fn make_property_factory(property: &'static str) -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let cell = Inscription::new(None);
        bind_item_property(item, property, &cell, "text");
        item.set_child(Some(&cell));
    });
    factory
}

pub fn make_icon_column(_key: &str, name: &str) -> ColumnViewColumn {
    let col_factory = SignalListItemFactory::new();
    col_factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let cell = gtk::Image::new();
        cell.set_icon_size(gtk::IconSize::Inherit);
        bind_item_property(item, PROP_ICON_NAME, &cell, "icon-name");
        item.set_child(Some(&cell));
    });

    let col = ColumnViewColumn::new(Some(name), Some(col_factory));
    col.set_resizable(false);
    col.set_expand(false);
    col.set_fixed_width(40);

    col
}

pub fn make_position_column(_key: &str, name: &str) -> ColumnViewColumn {
    let col = ColumnViewColumn::new(Some(name), Some(make_property_factory(PROP_QUEUE_POS)));
    col.set_resizable(false);
    col.set_expand(false);
    col.set_fixed_width(48);

    col
}

pub fn make_text_column(key: &str, name: &str, resizable: bool) -> ColumnViewColumn {
    let col_factory = if key == "duration" {
        make_property_factory(PROP_DURATION)
    } else {
        let col_factory = SignalListItemFactory::new();

        col_factory.connect_setup(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            item.set_child(Some(&Inscription::new(None)));
        });

        let field = key.to_string();

        col_factory.connect_bind(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let cell = item.child().and_downcast::<Inscription>().unwrap();
            let entry = item.item().and_downcast::<TrackItem>().unwrap();
            let track = entry.track();
            if field == "title" {
                cell.set_text(Some(track.display_title()));
            } else {
                cell.set_text(track.get_by_name(&field));
            }
        });

        col_factory
    };

    let col = ColumnViewColumn::new(Some(name), Some(col_factory));
    col.set_resizable(resizable);
    col.set_expand(true);

    col
}
