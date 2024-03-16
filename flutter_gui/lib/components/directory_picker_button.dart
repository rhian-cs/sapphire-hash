import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';

class DirectoryPickerButton extends StatelessWidget {
  final void Function(String) onPressed;
  const DirectoryPickerButton({super.key, required this.onPressed});

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: () async {
        String? selectedDirectory =
            await FilePicker.platform.getDirectoryPath();

        if (selectedDirectory != null) {
          onPressed(selectedDirectory);
        }
      },
      child: const Text('Choose Directory'),
    );
  }
}
