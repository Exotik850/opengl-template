mod app;
pub mod shape;
mod vertex;

use crate::app::App;

fn main() {
    let app = App::new();
    app.run();
}
