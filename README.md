# Tauri Setapp Plugin

Integrate [Setapp](https://setapp.com)'s framework into [Tauri](https://tauri.app) Mac apps

## Overview

The Setapp framework is automatically loaded when this plugin is initialized. This is achieved by calling a Swift bridge (SetappBridge) from Rust.

## Installation

### 1. Install the plugin

Add to `cargo.toml`:

```
[dependencies]
tauri-plugin-setapp = { git = "https://github.com/nlap/tauri-plugin-setapp.git", tag = "v2.0.0" }
```

### 2. Register the plugin

```
fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_setapp::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

### 3. Add your Setapp public key

[https://docs.setapp.com/docs/add-a-public-key-to-your-app-1](https://docs.setapp.com/docs/add-a-public-key-to-your-app-1)

In `tauri.conf.json`, add:

```
"resources": ["./setappPublicKey.pem"],
```

### 4. Update your Info.plist

[https://docs.setapp.com/docs/allow-setapp-to-update-your-app-on-macos-13](https://docs.setapp.com/docs/allow-setapp-to-update-your-app-on-macos-13)

In `src-tauri/Info.plist`:

```
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>NSUpdateSecurityPolicy</key>
    <dict>
        <key>AllowProcesses</key>
        <dict>
        <key>MEHY5QF425</key>
        <array>
            <string>com.setapp.DesktopClient.SetappAgent</string>
        </array>
        </dict>
    </dict>
</dict>
</plist>
```

### 5. Disable Tauri's updater

In `tauri.conf.json`:

```
{
  "tauri": {
    "updater": {
      "active": false
    }
  }
}
```

## Usage

Regular [app usage events](https://docs.setapp.com/docs/implement-usage-reporting) are automatically reported by Setapp framework. You can invoke other framework features from Javascript:

- ask_user_to_share_email
- show_release_notes_window_if_needed
- show_release_notes_window

```
invoke('plugin:setapp|ask_user_to_share_email', { });
```

## Cross-platform considerations

This plugin is cross-platform compatible and can be safely included in builds for any OS. On non-macOS platforms, the plugin remains inactive and has no effect.

## License

This code is provided as-is under the Apache License as a convenience for the Tauri community. I am not associated with MacPaw/Setapp.

Contributions are welcome!
