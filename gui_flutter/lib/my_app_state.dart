import 'package:flutter/material.dart';

class MyAppState extends ChangeNotifier {
  List<String> hashAlgorithms = ['SHA256', 'MD5', 'SHA1'];
  String selectedAlgorithm = '';

  MyAppState() {
    selectedAlgorithm = hashAlgorithms.first;
  }

  void selectAlgorithm(String algorithmName) {
    selectedAlgorithm = algorithmName;
    notifyListeners();
  }
}
