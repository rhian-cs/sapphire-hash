import 'package:flutter/material.dart';
import 'package:flutter_layout_grid/flutter_layout_grid.dart';
import 'package:hash_calculator/components/directory_picker_button.dart';
import 'package:hash_calculator/components/hash_algorithm_dropdown_menu.dart';

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
