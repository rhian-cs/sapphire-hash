import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (context) => MyAppState(),
      child: MaterialApp(
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
        home: const MyHomePage(),
      ),
    );
  }
}

class MyHomePage extends StatelessWidget {
  const MyHomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                const SizedBox(
                  width: 150,
                  child: Text('Calculate hashes for:'),
                ),
                DirectoryPickerButton(),
              ],
            ),
            const SizedBox(height: 5),
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                const SizedBox(
                  width: 150,
                  child: Text('Save CSV output to:'),
                ),
                DirectoryPickerButton(),
              ],
            ),
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                const SizedBox(
                  width: 150,
                  child: Text('Hash Algorithm:'),
                ),
                HashAlgorithmDropdownMenu()
              ],
            ),
            const SizedBox(height: 10),
            ElevatedButton(
              style: ElevatedButton.styleFrom(
                fixedSize: const Size(300, 35),
              ),
              onPressed: () {
                print("You've pressed the button!");
              },
              child: const Text('Calculate'),
            ),
          ],
        ),
      ),
    );
  }
}

class DirectoryPickerButton extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: () async {
        String? selectedDirectory =
            await FilePicker.platform.getDirectoryPath();

        if (selectedDirectory != null) {
          print("Directory picked: $selectedDirectory");
        }
      },
      child: const Text('Choose Directory'),
    );
  }
}

class HashAlgorithmDropdownMenu extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    var appState = context.watch<MyAppState>();

    return DropdownButton(
      value: appState.selectedAlgorithm,
      onChanged: (String? value) {
        appState.selectAlgorithm(value!);
      },
      items: appState.hashAlgorithms
          .map<DropdownMenuItem<String>>((algorithmName) {
        return DropdownMenuItem(
          value: algorithmName,
          child: Text(algorithmName),
        );
      }).toList(),
    );
  }
}

class MyAppState extends ChangeNotifier {
  List<String> hashAlgorithms = ['SHA256', 'MD5', 'SHA1'];
  String selectedAlgorithm = '';

  MyAppState() {
    selectedAlgorithm = hashAlgorithms.first;
  }

  void selectAlgorithm(String algorithmName) {
    selectedAlgorithm = algorithmName;
    notifyListeners();
  }
}
