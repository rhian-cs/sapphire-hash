import 'package:flutter/material.dart';
import 'package:hash_calculator/rust/api/hasher.dart';

class MyAppState extends ChangeNotifier {
  String? _inputDirectory;
  String? _outputDirectory;

  List<String> hashAlgorithms = ['SHA256', 'MD5', 'SHA1'];
  String selectedAlgorithm = '';
  bool isProcessing = false;

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

  void calculateHashes({required Function(String) onNotify}) {
    if (_inputDirectory == null) {
      onNotify("Please choose an input directory.");
      return;
    }

    if (_outputDirectory == null) {
      onNotify("Please choose an output directory.");
      return;
    }

    hasherProcess(
      directory: _inputDirectory!,
      hashAlgorithm: selectedAlgorithm.toLowerCase(),
      csvOutputDirectory: _outputDirectory!,
    ).then((value) {
      onNotify("${value.processedFilesCount} files were processed.\n\n"
          "Files were saved to:\n"
          "$_outputDirectory\n\n"
          "Took ${value.elapsedTimeSecs.toStringAsFixed(2)} seconds.");
    }).whenComplete(() {
      isProcessing = false;
      notifyListeners();
    });

    isProcessing = true;
    notifyListeners();
  }
}
