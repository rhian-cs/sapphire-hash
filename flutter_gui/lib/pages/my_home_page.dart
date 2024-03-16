import 'package:flutter/material.dart';
import 'package:flutter_layout_grid/flutter_layout_grid.dart';
import 'package:fluttertoast/fluttertoast.dart';
import 'package:sapphire_hash_gui/components/calculate_button.dart';
import 'package:sapphire_hash_gui/components/directory_picker_button.dart';
import 'package:sapphire_hash_gui/components/hash_algorithm_dropdown_menu.dart';
import 'package:sapphire_hash_gui/my_app_state.dart';
import 'package:provider/provider.dart';

class MyHomePage extends StatelessWidget {
  final FToast _fToast = FToast();

  MyHomePage({super.key});

  @override
  Widget build(BuildContext context) {
    var appState = context.watch<MyAppState>();
    _fToast.init(context);

    return Scaffold(
      body: Center(
        child: LayoutGrid(
          areas: '''
            inputDirectoryLabel      inputDirectoryButton
            outputDirectoryLabel     outputDirectoryButton
            hashAlgorithmSelectLabel hashAlgorithmSelectDropdown
            submitButton             submitButton
          ''',
          columnSizes: [150.px, 200.px],
          rowSizes: [40.px, 40.px, 40.px, 40.px],
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
              child: CalculateButton(
                onPressed: () => appState.calculateHashes(onNotify: _showToast),
                isProcessing: appState.isProcessing,
              ),
            ).inGridArea('submitButton'),
          ],
        ),
      ),
    );
  }

  _showToast(String message) {
    var toast = Container(
      padding: const EdgeInsets.symmetric(horizontal: 12.0, vertical: 6.0),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(20.0),
        color: Colors.primaries[3],
      ),
      child: Text(message),
    );

    _fToast.showToast(child: toast, fadeDuration: Durations.long3);
  }
}
