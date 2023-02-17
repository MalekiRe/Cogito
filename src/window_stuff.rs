use color_eyre::Result;
use crossbeam::channel::at;
use stereokit::Settings;
use captis::Capturer;
use x11rb::connection::Connection;
// use xcb::composite::{NameWindowPixmap, RedirectSubwindows};
// use xcb::{Connection, x, XidNew};
// use xcb::render::{CreatePicture, Pictformat, Picture, QueryPictFormats, QueryPictFormatsCookie};
// use xcb::x::{Drawable, GetWindowAttributes, GetWindowAttributesReply};

pub fn run() -> Result<()> {
    let capture = captis::init_capturer()?;
    let sk = Settings::default().init()?;



    sk.run(|sk| {
        let image = capture.capture_primary().unwrap();

    }, |_| {});
    Ok(())
    // let (conn, screen_num) = xcb::Connection::connect(None)?;
    // let setup = conn.get_setup();
    // let screen = setup.roots().nth(screen_num as usize).unwrap();
    //
    // let window = screen.root();
    // let thing = xcb::composite::RedirectSubwindows {
    //     window,
    //     update: xcb::composite::Redirect::Automatic,
    // };
    //
    // let cookie = conn.send_request_checked(&thing);
    //
    // conn.check_request(cookie)?;
    //
    // let cookie = conn.send_request(&GetWindowAttributes {
    //     window,
    // });
    //
    // let reply = conn.wait_for_reply(cookie)?;
    //
    //
    // let cookie = conn.send_request(&QueryPictFormats{});
    // let cookie = conn.wait_for_reply(cookie)?;
    // let format = cookie.formats().get(0).unwrap();
    // println!("{:#?}", format);
    // CreatePicture {
    //     pid: (),
    //     drawable: Drawable::None,
    //     format: format.id(),
    //     value_list: &[],
    // };
    // Ok(())
}