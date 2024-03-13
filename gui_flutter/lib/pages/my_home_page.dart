import 'package:flutter/material.dart';
import 'package:flutter_layout_grid/flutter_layout_grid.dart';
import 'package:hash_calculator/components/directory_picker_button.dart';
import 'package:hash_calculator/components/hash_algorithm_dropdown_menu.dart';
import 'package:hash_calculator/my_app_state.dart';
import 'package:provider/provider.dart';

class MyHomePage extends StatelessWidget {
  const MyHomePage({super.key});

  @override
  Widget build(BuildContext context) {
    var appState = context.watch<MyAppState>();

    return Scaffold(
      body: Center(
        child: LayoutGrid(
          areas: '''
            inputDirectoryLabel      inputDirectoryButton
            outputDirectoryLabel     outputDirectoryButton
            hashAlgorithmSelectLabel hashAlgorithmSelectDropdown
            submitButton             submitButton
            message                  message
          ''',
          columnSizes: [150.px, 200.px],
          rowSizes: [40.px, 40.px, 40.px, 40.px, 100.px],
          rowGap: 10,
          children: [
            const Text('Calculate hashes for:')
                .inGridArea('inputDirectoryLabel'),
            SizedBox(
              width: double.infinity,
              child: DirectoryPickerButton(
                onPressed: appState.setInputDirectory,
              ),
            ).inGridArea('inputDirectoryButton'),
            const Text('Save CSV output to:')
                .inGridArea('outputDirectoryLabel'),
            SizedBox(
              width: double.infinity,
              child: DirectoryPickerButton(
                onPressed: appState.setOutputDirectory,
              ),
            ).inGridArea('outputDirectoryButton'),
            const Text('Hash Algorithm:')
                .inGridArea('hashAlgorithmSelectLabel'),
            const HashAlgorithmDropdownMenu(isExpanded: true)
                .inGridArea('hashAlgorithmSelectDropdown'),
            SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                onPressed: appState.calculateHashes,
                child: const Text('Calculate'),
              ),
            ).inGridArea('submitButton'),
            Text(appState.message).inGridArea('message')
          ],
        ),
      ),
    );
  }
}
