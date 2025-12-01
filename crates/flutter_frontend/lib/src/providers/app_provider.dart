import 'dart:async';
import 'package:flutter/foundation.dart';
import '../models/models.dart';
import '../services/services.dart';

/// Application state provider
class AppProvider extends ChangeNotifier {
  List<Game> _games = [];
  final Map<String, Content> _gameContent = {};
  Game? _selectedGame;
  bool _isLoading = true;
  String? _error;
  Timer? _progressTimer;
  final Set<String> _activeDownloads = {};
  
  // Backend service (uses Rust backend or mock)
  final BackendService _backend = BackendFactory.instance;
  
  // Getters
  List<Game> get games => _games;
  Map<String, Content> get gameContent => _gameContent;
  Game? get selectedGame => _selectedGame;
  bool get isLoading => _isLoading;
  String? get error => _error;
  
  Content? getContent(String gameId) => _gameContent[gameId];
  
  /// Initialize app data
  Future<void> initialize() async {
    _isLoading = true;
    _error = null;
    notifyListeners();
    
    try {
      // Initialize the backend
      await _backend.initialize();
      
      // Load games from backend (Rust or mock)
      _games = await _backend.getGames();
      
      // Load content for each game
      for (final game in _games) {
        final content = await _backend.getGameContent(game.id, game.biz);
        if (content != null) {
          _gameContent[game.id] = content;
        }
      }
      
      // Select first game by default
      if (_games.isNotEmpty && _selectedGame == null) {
        _selectedGame = _games.first;
      }
      
      _isLoading = false;
      notifyListeners();
    } catch (e) {
      _error = e.toString();
      _isLoading = false;
      notifyListeners();
    }
  }
  
  /// Select a game
  void selectGame(Game game) {
    _selectedGame = game;
    notifyListeners();
  }
  
  /// Check if a game is installed
  bool isGameInstalled(String gameId) {
    final game = _games.where((g) => g.id == gameId).firstOrNull;
    if (game == null) return false;
    return _backend.isGameInstalled(gameId, game.biz);
  }
  
  /// Get download progress for a game
  DownloadProgress? getDownloadProgress(String gameId) {
    return _backend.getDownloadProgress(gameId);
  }
  
  /// Install/Download a game
  Future<void> installGame(String gameId) async {
    final game = _games.where((g) => g.id == gameId).firstOrNull;
    if (game == null) {
      debugPrint('Game not found: $gameId');
      return;
    }
    
    try {
      await _backend.installGame(gameId, game.biz);
      
      // Track this download and start polling for progress
      _activeDownloads.add(gameId);
      _startProgressPolling();
      
      notifyListeners();
    } catch (e) {
      debugPrint('Failed to start download: $e');
    }
  }
  
  /// Launch a game
  Future<void> launchGame(String gameId) async {
    await _backend.launchGame(gameId);
  }
  
  /// Start polling for download progress
  void _startProgressPolling() {
    // Cancel any existing timer
    _progressTimer?.cancel();
    
    // Poll every 500ms
    _progressTimer = Timer.periodic(const Duration(milliseconds: 500), (_) {
      _pollProgress();
    });
  }
  
  /// Stop polling for download progress
  void _stopProgressPolling() {
    _progressTimer?.cancel();
    _progressTimer = null;
  }
  
  /// Poll for download progress and notify listeners
  void _pollProgress() {
    if (_activeDownloads.isEmpty) {
      _stopProgressPolling();
      return;
    }
    
    // Check progress for all active downloads
    final completed = <String>[];
    
    for (final gameId in _activeDownloads) {
      final progress = _backend.getDownloadProgress(gameId);
      
      // Check if download is complete (no progress or not busy)
      if (progress == null || !progress.isBusy) {
        completed.add(gameId);
      }
    }
    
    // Remove completed downloads
    for (final gameId in completed) {
      _activeDownloads.remove(gameId);
      debugPrint('Download completed for game: $gameId');
    }
    
    // Stop polling if no more active downloads
    if (_activeDownloads.isEmpty) {
      _stopProgressPolling();
    }
    
    // Always notify to update UI
    notifyListeners();
  }
  
  @override
  void dispose() {
    _stopProgressPolling();
    super.dispose();
  }
}
