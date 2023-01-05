mod concurrency;
mod gui_library;
mod health_statistic;
mod library;
mod luhn_alogorithm;
mod polygon;

use gui_library::{Button, Label, Widget, Window};
fn main() {
    // gui_library::print_gui();
    // concurrency::start_thread();
    // concurrency::start_scope();
    // concurrency::channels();
    // concurrency::unbound_channel();
    // concurrency::bound_channel();
    concurrency::arc();
}
