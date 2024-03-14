# Recursive Hash Calculator (Flutter Frontend)

## Development

### Dependencies

- Flutter + Dart
- C++ build tools and other native dependencies

You can install the build tools and other native dependencies in Ubuntu with:

```sh
apt install cmake ninja-build clang pkg-config libgtk-3-dev
```

### Running the App

Inside `flutter_gui/` (this directory), run:

```sh
flutter run
```

### Building for Production

Inside `flutter_gui/` (this directory), run:

```sh
flutter build linux
```

### Updating Codegen

This app interacts with the Rust backend through the use of https://pub.dev/packages/flutter_rust_bridge.

Whenever you change the interface between the Rust core and the Flutter app (the `flutter_bridge` crate, essentially), you need to regenerate the bindings between the apps.

Inside `flutter_gui/` (this directory), run:

```sh
flutter_rust_bridge_codegen generate
```

### VS Code Development

If you want to use the debugger in VS Code you can use the following launch option:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Debug Flutter Frontend",
      "type": "dart",
      "request": "launch",
      "program": "flutter_gui/lib/main.dart"
    }
  ]
}
```

Use `"program": "flutter_gui/lib/main.dart"` if your VS Code workspace is at the root of the project and only `"program": "lib/main.dart"` if your VS Code workspace is inside `flutter_gui/`.
