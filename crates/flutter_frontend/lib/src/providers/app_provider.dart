import 'package:flutter/foundation.dart';
import '../models/models.dart';

/// Mock data for development - will be replaced by Rust bridge
class MockBackend {
  static Future<List<Game>> getGames() async {
    // Simulate network delay
    await Future.delayed(const Duration(milliseconds: 500));
    
    return [
      Game(
        id: '1Z8W5NHUQb',
        biz: 'hk4e_global',
        display: Display(
          language: 'en-us',
          name: 'Genshin Impact',
          icon: const Image(
            url: 'https://fastcdn.mihoyo.com/launcher-icon/1c6e65c1c6445fe52038e5d0c8cbed61.png',
            hoverUrl: '',
            link: '',
            md5: '',
            size: 0,
          ),
          title: 'Genshin Impact',
          subtitle: 'Open World RPG',
          background: const ImageLink(
            url: 'https://fastcdn.mihoyo.com/launcher-bg/20231115/0/bWFzc2Eg/lch_bg.jpg',
            link: '',
          ),
          logo: const ImageLink(
            url: 'https://fastcdn.mihoyo.com/launcher-bg/20231115/0/bWFzc2Eg/lch_logo.png',
            link: '',
          ),
          thumbnail: const ImageLink(url: '', link: ''),
          shortcut: const Image(url: '', hoverUrl: '', link: '', md5: '', size: 0),
        ),
        displayStatus: 'online',
      ),
      Game(
        id: 'gopR6Cufr3',
        biz: 'hkrpg_global',
        display: Display(
          language: 'en-us',
          name: 'Honkai: Star Rail',
          icon: const Image(
            url: 'https://fastcdn.mihoyo.com/launcher-icon/honkai-starrail.png',
            hoverUrl: '',
            link: '',
            md5: '',
            size: 0,
          ),
          title: 'Honkai: Star Rail',
          subtitle: 'Turn-based Space RPG',
          background: const ImageLink(
            url: 'https://fastcdn.mihoyo.com/launcher-bg/star-rail/star_rail_bg.jpg',
            link: '',
          ),
          logo: const ImageLink(
            url: 'https://fastcdn.mihoyo.com/launcher-bg/star-rail/star_rail_logo.png',
            link: '',
          ),
          thumbnail: const ImageLink(url: '', link: ''),
          shortcut: const Image(url: '', hoverUrl: '', link: '', md5: '', size: 0),
        ),
        displayStatus: 'online',
      ),
      Game(
        id: 'U5hbdsT9W7',
        biz: 'nap_global',
        display: Display(
          language: 'en-us',
          name: 'Zenless Zone Zero',
          icon: const Image(
            url: 'https://fastcdn.mihoyo.com/launcher-icon/zenless-zone-zero.png',
            hoverUrl: '',
            link: '',
            md5: '',
            size: 0,
          ),
          title: 'Zenless Zone Zero',
          subtitle: 'Urban Fantasy Action',
          background: const ImageLink(
            url: 'https://fastcdn.mihoyo.com/launcher-bg/zzz/zzz_bg.jpg',
            link: '',
          ),
          logo: const ImageLink(
            url: 'https://fastcdn.mihoyo.com/launcher-bg/zzz/zzz_logo.png',
            link: '',
          ),
          thumbnail: const ImageLink(url: '', link: ''),
          shortcut: const Image(url: '', hoverUrl: '', link: '', md5: '', size: 0),
        ),
        displayStatus: 'online',
      ),
      Game(
        id: 'zePXHT2t4L2tKR4m',
        biz: 'endfield',
        display: Display(
          language: 'en-us',
          name: 'Endfield',
          icon: const Image(
            url: 'https://play-lh.googleusercontent.com/l6FVNa293RykBWy88TqEhUakIcGSC8bRygSnKOBgztln48JX-WzMWnrBAETrKZsxDNC4HhwCsvfle_UI7rBE=w240-h480-rw',
            hoverUrl: '',
            link: '',
            md5: '',
            size: 0,
          ),
          title: 'Endfield',
          subtitle: 'Arknights Universe',
          background: const ImageLink(
            url: '',
            link: '',
          ),
          logo: const ImageLink(url: '', link: ''),
          thumbnail: const ImageLink(url: '', link: ''),
          shortcut: const Image(url: '', hoverUrl: '', link: '', md5: '', size: 0),
        ),
        displayStatus: 'online',
      ),
    ];
  }

  static Future<Content?> getGameContent(String gameId, String biz) async {
    await Future.delayed(const Duration(milliseconds: 300));
    
    // Return mock content with banners
    return Content(
      game: GameInfo(id: gameId, biz: biz),
      language: 'en-us',
      banners: [
        Banner(
          id: '1',
          image: const ImageLink(
            url: 'https://fastcdn.mihoyo.com/launcher-bg/20231115/0/bWFzc2Eg/banner1.jpg',
            link: '',
          ),
          i18nIdentifier: 'banner1',
        ),
      ],
      posts: [],
      socialMediaList: [],
    );
  }

  static bool isGameInstalled(String gameId) {
    // Mock: no games installed
    return false;
  }

  static DownloadProgress? getDownloadProgress(String gameId) {
    // Mock: no active downloads
    return null;
  }
}

/// Application state provider
class AppProvider extends ChangeNotifier {
  List<Game> _games = [];
  final Map<String, Content> _gameContent = {};
  Game? _selectedGame;
  bool _isLoading = true;
  String? _error;
  
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
      // Load games (will use Rust bridge in production)
      _games = await MockBackend.getGames();
      
      // Load content for each game
      for (final game in _games) {
        final content = await MockBackend.getGameContent(game.id, game.biz);
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
    return MockBackend.isGameInstalled(gameId);
  }
  
  /// Get download progress for a game
  DownloadProgress? getDownloadProgress(String gameId) {
    return MockBackend.getDownloadProgress(gameId);
  }
  
  /// Install/Download a game
  Future<void> installGame(String gameId) async {
    // TODO: Implement via Rust bridge
    debugPrint('Installing game: $gameId');
  }
  
  /// Launch a game
  Future<void> launchGame(String gameId) async {
    // TODO: Implement via Rust bridge
    debugPrint('Launching game: $gameId');
  }
}
