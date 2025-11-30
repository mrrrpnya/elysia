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
    await _backend.installGame(gameId, game.biz);
  }
  
  /// Launch a game
  Future<void> launchGame(String gameId) async {
    await _backend.launchGame(gameId);
  }
}
