import 'dart:io';
import 'package:flutter_cache_manager/flutter_cache_manager.dart';
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as p;

/// Custom cache manager for Elysia app images
/// Stores cached images in the app's data directory alongside other app data
class ElysiaCacheManager {
  static const key = 'elysiaImageCache';
  static CacheManager? _instance;
  
  static CacheManager get instance {
    _instance ??= CacheManager(
      Config(
        key,
        stalePeriod: const Duration(days: 30),
        maxNrOfCacheObjects: 200,
        repo: JsonCacheInfoRepository(databaseName: key),
        fileService: HttpFileService(),
      ),
    );
    return _instance!;
  }
  
  /// Initialize cache manager with custom directory
  static Future<void> initialize() async {
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
        fileService: HttpFileService(),
      ),
    );
  }
  
  /// Clear the image cache
  static Future<void> clearCache() async {
    await instance.emptyCache();
  }
}
