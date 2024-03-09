import 'package:flutter/material.dart';
import 'package:hash_calculator/my_app_state.dart';
import 'package:provider/provider.dart';

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
