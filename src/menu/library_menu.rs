use crate::library::Library;
use crate::{clear, sleep};

pub(crate) fn show(library: &Library) {
    clear();
    println!("Estamos em: {}.", library.get_name());
    sleep(1.5);
}
