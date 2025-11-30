// Real Rust backend service using flutter_rust_bridge
// This service calls into the Rust backend via FFI using JSON for complex data

import 'dart:convert';
import 'package:flutter/foundation.dart';
import '../models/models.dart';
import 'backend_service.dart';

// Import the generated FFI bindings
import '../rust/api.dart' as rust_api;
import '../rust/frb_generated.dart';

/// Rust backend service implementation using JSON serialization
class RustBackendService implements BackendService {
  bool _initialized = false;
  
  /// Initialize the Rust backend
  @override
  Future<void> initialize() async {
    if (_initialized) return;
    
    try {
      debugPrint('[RustBackend] Initializing Rust backend...');
      await RustLib.init();
      
      // Call init function
      final result = rust_api.initBackend();
      debugPrint('[RustBackend] Init result: $result');
      
      _initialized = true;
      debugPrint('[RustBackend] Rust backend initialized successfully');
    } catch (e) {
      debugPrint('[RustBackend] Failed to initialize: $e');
      rethrow;
    }
  }
  
  /// Get list of all games from Rust backend
  @override
  Future<List<Game>> getGames() async {
    await _ensureInitialized();
    
    try {
      debugPrint('[RustBackend] Fetching games...');
      
      // Call Rust API to get all games as JSON
      final jsonStr = await rust_api.getAllGamesJson();
      
      // Parse JSON
      final List<dynamic> jsonList = json.decode(jsonStr);
      
      // Convert to Game objects
      final games = jsonList.map((j) => _parseGameDto(j)).toList();
      
      debugPrint('[RustBackend] Fetched ${games.length} games');
      return games;
    } catch (e) {
      debugPrint('[RustBackend] Error fetching games: $e');
      rethrow;
    }
  }
  
  /// Get game content (banners, news, etc.)
  @override
  Future<Content?> getGameContent(String gameId, String biz) async {
    await _ensureInitialized();
    
    try {
      debugPrint('[RustBackend] Fetching content for game: $gameId');
      
      final jsonStr = await rust_api.getGameContentJson(gameId: gameId, biz: biz);
      
      if (jsonStr == 'null') {
        return null;
      }
      
      final jsonData = json.decode(jsonStr);
      return _parseContentDto(jsonData);
    } catch (e) {
      debugPrint('[RustBackend] Error fetching game content: $e');
      return null;
    }
  }
  
  /// Check if a game is installed
  @override
  bool isGameInstalled(String gameId, String biz) {
    if (!_initialized) return false;
    
    try {
      return rust_api.isGameInstalled(gameId: gameId, biz: biz);
    } catch (e) {
      debugPrint('[RustBackend] Error checking installation: $e');
      return false;
    }
  }
  
  /// Get download progress for a game
  @override
  DownloadProgress? getDownloadProgress(String gameId) {
    if (!_initialized) return null;
    
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
      debugPrint('[RustBackend] Error getting download progress: $e');
      return null;
    }
  }
  
  /// Install/download a game
  @override
  Future<void> installGame(String gameId, String biz) async {
    await _ensureInitialized();
    
    debugPrint('[RustBackend] Installing game: $gameId');
    // TODO: Implement game installation via Rust backend
  }
  
  /// Launch an installed game
  @override
  Future<void> launchGame(String gameId) async {
    await _ensureInitialized();
    
    debugPrint('[RustBackend] Launching game: $gameId');
    // TODO: Implement game launching via Rust backend
  }
  
  /// Ensure the backend is initialized
  Future<void> _ensureInitialized() async {
    if (!_initialized) {
      await initialize();
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
      id: b['id'] as String,
      image: ImageLink(
        url: b['image_url'] as String,
        link: b['link'] as String,
      ),
      i18nIdentifier: '',
    )).toList();
    
    final posts = (json['posts'] as List<dynamic>).map((p) => Post(
      id: p['id'] as String,
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
}
