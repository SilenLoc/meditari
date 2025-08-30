use crate::stoic::{display, entries};




pub fn daily_summary(owner: String) {
    let entries = entries::collect_all_entries(owner);
    display::simple_markdown(entries);
}
