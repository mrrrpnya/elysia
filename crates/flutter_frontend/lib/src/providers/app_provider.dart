import 'dart:async';
import 'dart:convert';
import 'package:flutter/foundation.dart';
import 'package:shared_preferences/shared_preferences.dart';
import '../models/models.dart';
import '../rust/api.dart' as rust_api;
import '../rust/frb_generated.dart';

/// Application state provider
class AppProvider extends ChangeNotifier {
  static const String _lastSelectedGameKey = 'last_selected_game_id';
  
  List<Game> _games = [];
  final Map<String, Content> _gameContent = {};
  Game? _selectedGame;
  bool _isLoading = true;
  String? _error;
  Timer? _progressTimer;
  final Set<String> _activeDownloads = {};
  SharedPreferences? _prefs;
  bool _backendInitialized = false;
  
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
      // Initialize shared preferences for persistent storage
      try {
        _prefs = await SharedPreferences.getInstance();
      } catch (e) {
        debugPrint('Failed to initialize SharedPreferences: $e');
        _prefs = null;
      }
      
      // Initialize the Rust backend
      await _initializeBackend();
      
      // Load games from Rust backend
      _games = await _getGames();
      
      // Load content for each game
      for (final game in _games) {
        final content = await _getGameContent(game.id, game.biz);
        if (content != null) {
          _gameContent[game.id] = content;
        }
      }
      
      // Restore last selected game or select first game
      await _restoreLastSelectedGame();
      
      _isLoading = false;
      notifyListeners();
    } catch (e) {
      _error = e.toString();
      _isLoading = false;
      notifyListeners();
    }
  }
  
  /// Initialize the Rust backend
  Future<void> _initializeBackend() async {
    if (_backendInitialized) return;
    
    try {
      debugPrint('[AppProvider] Initializing Rust backend...');
      await RustLib.init();
      
      final result = rust_api.initBackend();
      debugPrint('[AppProvider] Init result: $result');
      
      _backendInitialized = true;
      debugPrint('[AppProvider] Rust backend initialized successfully');
    } catch (e) {
      debugPrint('[AppProvider] Failed to initialize: $e');
      rethrow;
    }
  }
  
  /// Get list of all games from Rust backend
  Future<List<Game>> _getGames() async {
    try {
      debugPrint('[AppProvider] Fetching games...');
      
      final jsonStr = await rust_api.getAllGamesJson();
      final List<dynamic> jsonList = json.decode(jsonStr);
      
      final games = jsonList.map((j) => _parseGameDto(j as Map<String, dynamic>)).toList();
      
      debugPrint('[AppProvider] Fetched ${games.length} games');
      return games;
    } catch (e) {
      debugPrint('[AppProvider] Error fetching games: $e');
      rethrow;
    }
  }
  
  /// Get game content (banners, news, etc.)
  Future<Content?> _getGameContent(String gameId, String biz) async {
    try {
      debugPrint('[AppProvider] Fetching content for game: $gameId');
      
      final jsonStr = await rust_api.getGameContentJson(gameId: gameId, biz: biz);
      
      if (jsonStr == 'null') {
        return null;
      }
      
      final jsonData = json.decode(jsonStr);
      return _parseContentDto(jsonData as Map<String, dynamic>);
    } catch (e) {
      debugPrint('[AppProvider] Error fetching game content: $e');
      return null;
    }
  }
  
  /// Parse GameDto JSON to Game model
  Game _parseGameDto(Map<String, dynamic> json) {
    return Game(
      id: json['id'] as String,
      biz: json['biz'] as String,
      display: Display(
        language: 'en-us',
        name: json['name'] as String,
        icon: Image(
          url: json['icon_url'] as String,
          hoverUrl: '',
          link: '',
          md5: '',
          size: 0,
        ),
        title: json['title'] as String,
        subtitle: json['subtitle'] as String,
        background: ImageLink(
          url: json['background_url'] as String,
          link: '',
        ),
        logo: ImageLink(
          url: json['logo_url'] as String,
          link: '',
        ),
        thumbnail: const ImageLink(url: '', link: ''),
        shortcut: const Image(url: '', hoverUrl: '', link: '', md5: '', size: 0),
      ),
      displayStatus: json['display_status'] as String,
    );
  }
  
  /// Parse ContentDto JSON to Content model
  Content _parseContentDto(Map<String, dynamic> json) {
    final banners = (json['banners'] as List<dynamic>).map((b) => Banner(
      id: (b as Map<String, dynamic>)['id'] as String,
      image: ImageLink(
        url: b['image_url'] as String,
        link: b['link'] as String,
      ),
      i18nIdentifier: '',
    )).toList();
    
    final posts = (json['posts'] as List<dynamic>).map((p) => Post(
      id: (p as Map<String, dynamic>)['id'] as String,
      postType: p['post_type'] as String,
      title: p['title'] as String,
      link: p['link'] as String,
      date: p['date'] as String,
    )).toList();
    
    return Content(
      game: GameInfo(
        id: json['game_id'] as String,
        biz: json['game_biz'] as String,
      ),
      language: json['language'] as String,
      banners: banners,
      posts: posts,
      socialMediaList: [],
    );
  }
  
  /// Restore the last selected game from preferences
  Future<void> _restoreLastSelectedGame() async {
    if (_games.isEmpty) return;
    
    try {
      final lastGameId = _prefs?.getString(_lastSelectedGameKey);
      if (lastGameId != null) {
        // Find the game with the saved ID
        final lastGame = _games.where((g) => g.id == lastGameId).firstOrNull;
        if (lastGame != null) {
          _selectedGame = lastGame;
          return;
        }
      }
    } catch (e) {
      debugPrint('Failed to restore last selected game: $e');
    }
    
    // Default to first game if no saved selection or game not found
    _selectedGame = _games.first;
  }
  
  /// Save the selected game ID to preferences
  Future<void> _saveSelectedGame(String gameId) async {
    try {
      await _prefs?.setString(_lastSelectedGameKey, gameId);
    } catch (e) {
      debugPrint('Failed to save selected game: $e');
    }
  }
  
  /// Select a game
  void selectGame(Game game) {
    _selectedGame = game;
    // Fire and forget - don't block UI for persistence
    _saveSelectedGame(game.id);
    notifyListeners();
  }
  
  /// Check if a game is installed
  bool isGameInstalled(String gameId) {
    if (!_backendInitialized) return false;
    
    final game = _games.where((g) => g.id == gameId).firstOrNull;
    if (game == null) return false;
    
    try {
      return rust_api.isGameInstalled(gameId: gameId, biz: game.biz);
    } catch (e) {
      debugPrint('[AppProvider] Error checking installation: $e');
      return false;
    }
  }
  
  /// Get download progress for a game
  DownloadProgress? getDownloadProgress(String gameId) {
    if (!_backendInitialized) return null;
    
    try {
      final jsonStr = rust_api.getDownloadProgressJson(gameId: gameId);
      
      if (jsonStr == 'null') {
        return null;
      }
      
      final jsonData = json.decode(jsonStr);
      return DownloadProgress(
        downloaded: jsonData['downloaded'] as int,
        total: jsonData['total'] as int,
        mbPerSecond: (jsonData['mb_per_second'] as num).toDouble(),
        partIndex: jsonData['part_index'] as int,
        partsTotal: jsonData['parts_total'] as int,
        status: jsonData['status'] as String,
        isBusy: jsonData['is_busy'] as bool,
      );
    } catch (e) {
      debugPrint('[AppProvider] Error getting download progress: $e');
      return null;
    }
  }
  
  /// Install/Download a game
  Future<void> installGame(String gameId) async {
    final game = _games.where((g) => g.id == gameId).firstOrNull;
    if (game == null) {
      debugPrint('Game not found: $gameId');
      return;
    }
    
    try {
      debugPrint('[AppProvider] Installing game: $gameId (biz: ${game.biz})');
      
      final result = await rust_api.installGame(gameId: gameId, biz: game.biz);
      if (result != 'ok') {
        debugPrint('[AppProvider] Installation error: $result');
        throw Exception(result);
      }
      debugPrint('[AppProvider] Installation started for game: $gameId');
      
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
    try {
      debugPrint('[AppProvider] Launching game: $gameId');
      
      final result = await rust_api.launchGame(gameId: gameId);
      if (result != 'ok') {
        debugPrint('[AppProvider] Launch error: $result');
        throw Exception(result);
      }
      debugPrint('[AppProvider] Game launched: $gameId');
    } catch (e) {
      debugPrint('[AppProvider] Failed to launch game: $e');
      rethrow;
    }
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
      final progress = getDownloadProgress(gameId);
      
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
