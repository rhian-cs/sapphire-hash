import 'package:flutter/material.dart';
import 'package:fluttertoast/fluttertoast.dart';
import 'package:hash_calculator/my_app_state.dart';
import 'package:hash_calculator/pages/my_home_page.dart';
import 'package:provider/provider.dart';

import 'package:hash_calculator/rust/frb_generated.dart';
import 'package:window_manager/window_manager.dart';

Future<void> main() async {
  await initializeWindow();
  await RustLib.init();

  runApp(const MyApp());
}

Future<void> initializeWindow() async {
  WidgetsFlutterBinding.ensureInitialized();
  await windowManager.ensureInitialized();

  WindowOptions windowOptions = const WindowOptions(
    size: Size(640, 480),
  );

  windowManager.waitUntilReadyToShow(windowOptions, () async {
    await windowManager.show();
  });
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (context) => MyAppState(),
      child: MaterialApp(
        builder: FToastBuilder(),
        title: 'Hash Calculator',
        theme: ThemeData(
          brightness: Brightness.light,
          colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
          useMaterial3: true,
        ),
        darkTheme: ThemeData(
          brightness: Brightness.dark,
        ),
        themeMode: ThemeMode.dark,
        home: MyHomePage(),
      ),
    );
  }
}
