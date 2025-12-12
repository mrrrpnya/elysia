import 'dart:async';
import 'dart:convert';
import 'package:flutter/foundation.dart';
import 'package:shared_preferences/shared_preferences.dart';
import '../models/models.dart';
import 'package:elysia/api.dart' as rust_api;
import 'package:elysia/frb_generated.dart';

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
      
      // New: Returns List<GameDto> directly, no JSON parsing needed!
      final games = await rust_api.getAllGames();
      final games = games.map((game) => _gameFromRust(game)).toList();
      
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
      
      // New: Returns ContentDto? directly, no JSON parsing needed!
      final content = await rust_api.getGameContent(gameId: gameId, biz: biz);
      
      if (content == null) {
        return null;
      }
      
      return _contentFromRust(content);
    } catch (e) {
      debugPrint('[AppProvider] Error fetching game content: $e');
      return null;
    }
  }
  
  /// Convert GameDto to Game model
  Game _gameFromRust(dynamic game) {
    return Game(
      id: game.id,
      biz: game.biz,
      display: Display(
        language: 'en-us',
        name: game.name,
        icon: Image(
          url: game.iconUrl,
          hoverUrl: '',
          link: '',
          md5: '',
          size: 0,
        ),
        title: game.title,
        subtitle: game.subtitle,
        background: ImageLink(
          url: game.backgroundUrl,
          link: '',
        ),
        logo: ImageLink(
          url: game.logoUrl,
          link: '',
        ),
        thumbnail: const ImageLink(url: '', link: ''),
        shortcut: const Image(url: '', hoverUrl: '', link: '', md5: '', size: 0),
        videoBackgroundUrl: game.videoBackgroundUrl,
        themeImageUrl: game.themeImageUrl,
        backgroundType: game.backgroundType,
      ),
      displayStatus: game.displayStatus,
    );
  }
  
  /// Convert ContentDto to Content model
  Content _contentFromRust(dynamic content) {
    final banners = content.banners.map<Banner>((b) => Banner(
      id: b.id,
      image: ImageLink(
        url: b.imageUrl,
        link: b.link,
      ),
      i18nIdentifier: '',
    )).toList();
    
    final posts = content.posts.map<Post>((p) => Post(
      id: p.id,
      postType: p.postType,
      title: p.title,
      link: p.link,
      date: p.date,
    )).toList();
    
    return Content(
      game: GameInfo(
        id: content.gameId,
        biz: content.gameBiz,
      ),
      language: content.language,
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
      // New: Returns DownloadProgressDto? directly!
      final progress = rust_api.getDownloadProgress(gameId: gameId);
      
      if (progress == null) {
        return null;
      }
      
      return DownloadProgress(
        downloaded: progress.downloaded,
        total: progress.total,
        mbPerSecond: progress.mbPerSecond.toDouble(),
        partIndex: progress.partIndex,
        partsTotal: progress.partsTotal,
        status: progress.status,
        isBusy: progress.isBusy,
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
