/*
extern crate iui;
use iui::controls::{Button, Group, Label, VerticalBox};
use iui::prelude::*;
*/

use std::thread::sleep;
use std::time::Duration;

use notify_rust::Notification;
use notify_rust::Timeout;



fn main() {
    let mut count1 = 0;
    //let mut time_in_seconds = 0;

    Notification::new()
        .summary("Welcome back sir.")
        .body("I wish you a well and rich coding session.")
        .timeout(Timeout::Milliseconds(6000))
        .show()
        .unwrap();

    println!("Welcome back sir.");
    /*
    let ui = UI::init().expect("Failed to initialize UI");
    let mut win = Window::new(&ui, "Test Window", 200, 200, WindowType::NoMenubar);

        let mut vbox = VerticalBox::new(&ui);
        vbox.set_padded(&ui, true);

        let mut button = Button::new(&ui, "TestButton");
        button.on_clicked(&ui, {
            let ui = ui.clone();
            move |btn| {
                btn.set_text(&ui, "Clicked!");
            }
        });

        win.set_child(&ui, &vbox);
        win.show(&ui);
        win.main();
        */

    loop {
        sleep(Duration::from_secs(600));

        if count1 != 2 || count1 != 4 || count1 != 6 {
            Notification::new()
            .summary("Straighten your back")
                .body("You seen these coders and their turtle neck. Be different and have a good posture!")
                .timeout(Timeout::Milliseconds(10000))
                .show()
                .unwrap();

            println!("Straighten your back");
        }

        if count1 == 2 || count1 == 4 {
            Notification::new()
            .summary("Take a glass of water")
            .body("Staying hydrated is a good way to both clear your mind and stay in the game. So do it!")
                .timeout(Timeout::Milliseconds(10000))
                .show()
                .unwrap();

            println!("Take a glass of water");
        }

        
        if count1 >= 6 {
            Notification::new()
            .summary("Take a quick rest. (5 to 15 minutes)")
            .body("You need rest to always have a clear mind. So take one, you deserve it!")
            .timeout(Timeout::Milliseconds(10000))
            .show()
            .unwrap();
            
            count1 = 0;
            
            println!("Take a quick rest. (5 to 15 minutes)");
        }
        count1 += 1;
    }
}
