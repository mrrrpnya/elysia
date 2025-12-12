// Extensions for auto-generated API types
// Adds computed properties and helper methods

import 'package:elysia/api.dart' as api;

/// Extensions for DownloadProgress
extension DownloadProgressExtensions on api.DownloadProgress {
  /// Calculate download percentage
  double get percentage => total > 0 ? (downloaded / total) * 100 : 0;
  
  /// Get downloaded size in GB
  String get downloadedGb => (downloaded / 1000000000).toStringAsFixed(2);
  
  /// Get total size in GB
  String get totalGb => (total / 1000000000).toStringAsFixed(2);
}

/// Extensions for Game
extension GameExtensions on api.Game {
  /// Check if game has a video background
  bool get hasVideoBackground => 
      backgroundType == 'BACKGROUND_TYPE_VIDEO' && videoBackgroundUrl.isNotEmpty;
}
