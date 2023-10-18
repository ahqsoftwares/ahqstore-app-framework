use dirs_next::cache_dir;

use wry::{
    application::{
        dpi::{LogicalSize, PhysicalSize},
        event_loop::{EventLoop, EventLoopBuilder},
        platform::windows::{IconExtWindows, WindowExtWindows},
        window::Icon,
        window::WindowBuilder,
    },
    webview::{WebContext, WebViewBuilder},
};

use windows::{
    core::BSTR,
    Win32::Foundation::HWND,
    Win32::System::Com::StructuredStorage::PROPVARIANT,
    Win32::System::Variant::VT_BSTR,
    Win32::UI::Shell::PropertiesSystem::SHGetPropertyStoreForWindow,
    Win32::UI::Shell::PropertiesSystem::{IPropertyStore, PROPERTYKEY},
};

fn main() {
    let app: EventLoop<String> = EventLoopBuilder::with_user_event().build();

    let _proxy = app.create_proxy();

    let a = WindowBuilder::new()
        .with_window_icon(
            Icon::from_path(
                r#"E:\GitHub\ahq-store-tauri\public\favicon.ico"#,
                Some(PhysicalSize::from_logical(
                    LogicalSize::new(128.0, 128.0),
                    1.0,
                )),
            )
            .map_or_else(|_| None, |x| Some(x)),
        )
        .build(&app)
        .unwrap();

    let a = WebViewBuilder::new(a)
        .unwrap()
        .with_devtools(false)
        .with_incognito(true)
        .with_web_context(&mut WebContext::new(
            cache_dir().map_or_else(|| None, |x| Some(x.join("Temp"))),
        ))
        .build()
        .unwrap();

    a.load_url("https://google.com");

    unsafe {
        let handle = a.window().hwnd();
        let store = SHGetPropertyStoreForWindow::<HWND, IPropertyStore>(HWND(handle)).unwrap();

        let mut model_id = PROPVARIANT::default();

        let my_string = "com.ahqsoftwares.app";

        model_id.Anonymous.decVal.wReserved = VT_BSTR.0;
        *(*model_id.Anonymous.Anonymous).Anonymous.bstrVal = BSTR::from(my_string);

        let model_id: *mut PROPVARIANT = &mut model_id as *mut _;

        let key = PROPERTYKEY {
            fmtid: windows::core::GUID::from_values(
                0x9F4C2855,
                0x9F79,
                0x4B39,
                [0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3],
            ),
            pid: 5,
        };

        store.SetValue(&key as *const _, model_id).unwrap();
    }

    let b = WindowBuilder::new().build(&app).unwrap();

    let b = WebViewBuilder::new(b)
        .unwrap()
        .with_devtools(false)
        .with_incognito(true)
        .with_web_context(&mut WebContext::new(
            cache_dir().map_or_else(|| None, |x| Some(x.join("Temp"))),
        ))
        .build()
        .unwrap();

    b.load_url("https://google.com");

    unsafe {
        let handle = b.window().hwnd();
        let store = SHGetPropertyStoreForWindow::<HWND, IPropertyStore>(HWND(handle)).unwrap();

        let my_string = "com.ahqsoftwares.app2";

        let mut model_id = PROPVARIANT::default();

        *(*model_id.Anonymous.Anonymous).Anonymous.bstrVal = BSTR::from(my_string);

        let model_id: *const PROPVARIANT = &model_id as *const _;

        let key = PROPERTYKEY {
            fmtid: windows::core::GUID::from_values(
                0x9F4C2855,
                0x9F79,
                0x4B39,
                [0xA8, 0xD0, 0xE1, 0xD4, 0x2D, 0xE1, 0xD5, 0xF3],
            ),
            pid: 5,
        };
    }

    app.run(|a, b, c| match a {
        _ => {}
    });
}
