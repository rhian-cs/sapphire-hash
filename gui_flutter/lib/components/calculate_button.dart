import 'package:flutter/material.dart';

class CalculateButton extends StatelessWidget {
  final bool isProcessing;
  final void Function()? onPressed;

  const CalculateButton(
      {super.key, required this.isProcessing, required this.onPressed});

  @override
  Widget build(BuildContext context) {
    var buttonText = isProcessing ? 'Please wait...' : 'Calculate';

    return IgnorePointer(
        ignoring: isProcessing,
        child: ElevatedButton(
          onPressed: onPressed,
          child: Text(buttonText),
        ));
  }
}
