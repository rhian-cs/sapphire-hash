import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';

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
