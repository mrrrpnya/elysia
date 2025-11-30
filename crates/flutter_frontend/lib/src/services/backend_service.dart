// Backend service abstraction layer
// This provides a unified interface that can use either the mock backend
// (for development) or the real Rust backend via flutter_rust_bridge

import 'package:flutter/foundation.dart';
import '../models/models.dart';
import 'mock_backend_service.dart';
import 'rust_backend_service.dart';

/// Abstract backend service interface
abstract class BackendService {
  Future<List<Game>> getGames();
  Future<Content?> getGameContent(String gameId, String biz);
  bool isGameInstalled(String gameId, String biz);
  DownloadProgress? getDownloadProgress(String gameId);
  Future<void> installGame(String gameId, String biz);
  Future<void> launchGame(String gameId);
  Future<void> initialize();
}

/// Factory to get the appropriate backend service
class BackendFactory {
  static BackendService? _instance;
  
  static BackendService get instance {
    _instance ??= _createBackend();
    return _instance!;
  }
  
  static BackendService _createBackend() {
    // Try to use the real Rust backend, fall back to mock if unavailable
    try {
      return RustBackendService();
    } catch (e) {
      debugPrint('[BackendFactory] Rust backend unavailable, using mock: $e');
      return MockBackendService();
    }
  }
  
  /// Force use of mock backend (for testing)
  static void useMockBackend() {
    _instance = MockBackendService();
  }
  
  /// Force use of Rust backend
  static void useRustBackend() {
    _instance = RustBackendService();
  }
}
