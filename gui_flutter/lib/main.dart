import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_layout_grid/flutter_layout_grid.dart';
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
        child: LayoutGrid(
          areas: '''
            inputDirectoryLabel      inputDirectoryButton
            outputDirectoryLabel     outputDirectoryButton
            hashAlgorithmSelectLabel hashAlgorithmSelectDropdown
            submitButton             submitButton
          ''',
          columnSizes: [150.px, 160.px],
          rowSizes: [40.px, 40.px, 40.px, 40.px],
          rowGap: 10,
          children: [
            const Text('Calculate hashes for:')
                .inGridArea('inputDirectoryLabel'),
            const SizedBox(
              width: double.infinity,
              child: DirectoryPickerButton(),
            ).inGridArea('inputDirectoryButton'),
            const Text('Save CSV output to:')
                .inGridArea('outputDirectoryLabel'),
            const SizedBox(
              width: double.infinity,
              child: DirectoryPickerButton(),
            ).inGridArea('outputDirectoryButton'),
            const Text('Hash Algorithm:')
                .inGridArea('hashAlgorithmSelectLabel'),
            const HashAlgorithmDropdownMenu(isExpanded: true)
                .inGridArea('hashAlgorithmSelectDropdown'),
            SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                onPressed: () {
                  print("You've pressed the button!");
                },
                child: const Text('Calculate'),
              ),
            ).inGridArea('submitButton'),
          ],
        ),
      ),
    );
  }
}

class DirectoryPickerButton extends StatelessWidget {
  const DirectoryPickerButton({super.key});

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
  final bool isExpanded;

  const HashAlgorithmDropdownMenu({super.key, required this.isExpanded});

  @override
  Widget build(BuildContext context) {
    var appState = context.watch<MyAppState>();

    return DropdownButton(
      isExpanded: isExpanded,
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
