import 'dart:io';
import 'package:flutter_cache_manager/flutter_cache_manager.dart';
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as p;

/// Custom cache manager for Elysia app images
/// Stores cached images in the app's data directory alongside other app data
class ElysiaCacheManager {
  static const key = 'elysiaImageCache';
  static CacheManager? _instance;
  static bool _initialized = false;

  static CacheManager get instance {
    // Return default cache manager if not initialized yet
    // This prevents crashes if images try to load before initialization
    if (!_initialized || _instance == null) {
      return DefaultCacheManager();
    }
    return _instance!;
  }

  /// Initialize cache manager with custom directory in app's data folder
  static Future<void> initialize() async {
    if (_initialized) return;

    try {
      // Get the app's support directory
      final appDir = await getApplicationSupportDirectory();
      final cacheDir = Directory(p.join(appDir.path, 'image_cache'));

      // Create cache directory if it doesn't exist
      if (!await cacheDir.exists()) {
        await cacheDir.create(recursive: true);
      }

      _instance = CacheManager(
        Config(
          key,
          stalePeriod: const Duration(days: 30),
          maxNrOfCacheObjects: 200,
          repo: JsonCacheInfoRepository(databaseName: key),
          fileSystem: IOFileSystem(cacheDir.path),
          fileService: HttpFileService(),
        ),
      );
      _initialized = true;
    } catch (e) {
      // If initialization fails, use default cache manager
      _instance = null;
      _initialized = false;
    }
  }

  /// Clear the image cache
  static Future<void> clearCache() async {
    await instance.emptyCache();
  }
}
