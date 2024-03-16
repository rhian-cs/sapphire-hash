import 'package:flutter/material.dart';
import 'package:sapphire_hash_gui/rust/api/hasher.dart';
import 'package:intl/intl.dart';

class MyAppState extends ChangeNotifier {
  String? _inputDirectory;
  String? _outputDirectory;

  final DateFormat _dateFormat = DateFormat('yyyy-MM-dd-HH-mm-ss');

  List<String> hashAlgorithms = [];
  String selectedAlgorithm = '';
  bool isProcessing = false;

  MyAppState() {
    hashAlgorithms = availableHashingAlgorithms();
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

    var csvOutputFilename = _csvOutputFilename(_outputDirectory!);

    hasherProcess(
      directory: _inputDirectory!,
      hashAlgorithm: selectedAlgorithm.toLowerCase(),
      csvOutputFilename: csvOutputFilename,
    ).then((value) {
      onNotify("${value.processedFilesCount} files were processed.\n\n"
          "Files were saved to:\n"
          "$csvOutputFilename\n\n"
          "Took ${value.elapsedTimeSecs.toStringAsFixed(2)} seconds.");
    }).whenComplete(() {
      isProcessing = false;
      notifyListeners();
    });

    isProcessing = true;
    notifyListeners();
  }

  String _csvOutputFilename(String outputDirectory) {
    String formattedTime = _dateFormat.format(DateTime.now());

    return "$outputDirectory/hasher-report-$formattedTime-$selectedAlgorithm.csv";
  }
}
