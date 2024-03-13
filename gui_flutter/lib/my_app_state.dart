import 'package:flutter/material.dart';
import 'package:hash_calculator/rust/api/hasher.dart';

class MyAppState extends ChangeNotifier {
  String? _inputDirectory;
  String? _outputDirectory;
  String message = "Click to see results.";

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
    if (_inputDirectory == null) {
      message = "Please choose an input directory.";
      notifyListeners();
      return;
    }

    if (_outputDirectory == null) {
      message = "Please choose an output directory.";
      notifyListeners();
      return;
    }

    var result = hasherProcess(
      directory: _inputDirectory!,
      hashAlgorithm: selectedAlgorithm.toLowerCase(),
      csvOutputDirectory: _outputDirectory!,
    );

    message = "Now processing...";

    notifyListeners();

    result.then((value) {
      message =
          "${value.processedFilesCount} files were processed. Took ${value.elapsedTimeSecs} seconds.";
      notifyListeners();
    });
  }
}
