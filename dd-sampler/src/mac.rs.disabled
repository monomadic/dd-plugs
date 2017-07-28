use vst2::editor::{Editor};

use libc::c_void;

use std::fs::File;
use std::ops::Deref;
use simplelog::*;

use cocoa;

use cocoa::base::{selector, id, nil, NO};

use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize, NSAutoreleasePool, NSProcessInfo,
                        NSString};
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSWindow,
                    NSTitledWindowMask, NSBackingStoreBuffered, NSClosableWindowMask,
                    NSResizableWindowMask, NSMiniaturizableWindowMask,
                    NSUnifiedTitleAndToolbarWindowMask, NSMenu, NSMenuItem, NSTabView,
                    NSTabViewItem, NSRunningApplication, NSApplicationActivateIgnoringOtherApps, NSView};

#[derive(Default)]
pub struct Interface {
    is_open: bool,
}

impl Interface {
    pub fn new() -> Interface {
        let _ = CombinedLogger::init(
            vec![
                // TermLogger::new( LevelFilter::Warn, Config::default()).unwrap(),
                WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("/tmp/simplesynth.log").unwrap()),
            ]
        );

        Interface{ is_open: false }
    }
}

impl Editor for Interface {
    fn size(&self) -> (i32, i32) {
        (500, 500)
    }

    fn position(&self) -> (i32, i32) {
        (500, 500)
    }

    fn open(&mut self, window: *mut c_void) {
        // use objc::runtime::{Object, Class};

        unsafe {
            let f = (window as id).bounds() as NSRect;
            let s = f.size.width;
            info!("{}", s);
        }
    }

    fn is_open(&mut self) -> bool {
        self.is_open
    }
}
