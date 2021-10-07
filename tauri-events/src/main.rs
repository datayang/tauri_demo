use bindings::{
    Windows::Foundation::Uri,
    Windows::Web::Syndication::SyndicationClient,
    Windows::Win32::UI::WindowsAndMessaging::*,
};

fn main() -> windows::Result<()> {
    // println!("Hello, world!");
    // let uri = Uri::CreateUri("https://blogs.windows.com/feed")?;
    // let client = SyndicationClient::new()?;
    // let feed = client.RetrieveFeedAsync(uri)?.get()?;
    // for item in feed.Items()? {
    //     println!("{}", item.Title()?.Text()?);
    // }
    unsafe {
        MessageBoxA(None, "ni hao", "Caption", MB_OK);
    }
    
    Ok(())
}
