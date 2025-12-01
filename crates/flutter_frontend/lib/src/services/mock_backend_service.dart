// Mock backend service for development without Rust
// This is used as a fallback when the Rust backend is not available

import 'package:flutter/foundation.dart';
import '../models/models.dart';
import 'backend_service.dart';

/// Mock backend service implementation
class MockBackendService implements BackendService {
  bool _initialized = false;
  
  @override
  Future<void> initialize() async {
    if (_initialized) return;
    debugPrint('[MockBackend] Initializing mock backend');
    await Future.delayed(const Duration(milliseconds: 100));
    _initialized = true;
  }
  
  @override
  Future<List<Game>> getGames() async {
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

  @override
  Future<Content?> getGameContent(String gameId, String biz) async {
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

  @override
  bool isGameInstalled(String gameId, String biz) {
    // Mock: no games installed
    return false;
  }

  @override
  DownloadProgress? getDownloadProgress(String gameId) {
    // Mock: no active downloads
    return null;
  }
  
  @override
  Future<void> installGame(String gameId, String biz) async {
    debugPrint('[MockBackend] Installing game: $gameId (mock)');
  }
  
  @override
  Future<void> launchGame(String gameId) async {
    debugPrint('[MockBackend] Launching game: $gameId (mock)');
  }
}
