import 'package:flutter/material.dart';

class MyAppState extends ChangeNotifier {
  String? _inputDirectory;
  String? _outputDirectory;

  List<String> hashAlgorithms = ['SHA256', 'MD5', 'SHA1'];
  String selectedAlgorithm = '';

  MyAppState() {
    selectedAlgorithm = hashAlgorithms.first;
  }

  void selectAlgorithm(String algorithmName) {
    selectedAlgorithm = algorithmName;
    notifyListeners();
  }

  void setInputDirectory(String directory) {
    _inputDirectory = directory;
    notifyListeners();
  }

  void setOutputDirectory(String directory) {
    _outputDirectory = directory;
    notifyListeners();
  }

  void calculateHashes() {
    print(
        "Now calculating hashes for files in $_inputDirectory. Output will be saved in $_outputDirectory.");
    notifyListeners();
  }
}
