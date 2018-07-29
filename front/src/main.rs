extern crate yew;
extern crate app;

use yew::prelude::*;
use app::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
