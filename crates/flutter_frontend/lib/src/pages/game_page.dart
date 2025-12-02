import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:cached_network_image/cached_network_image.dart';
import 'package:media_kit/media_kit.dart';
import 'package:media_kit_video/media_kit_video.dart';
import '../models/models.dart';
import '../providers/app_provider.dart';
import '../theme/theme.dart';
import '../widgets/widgets.dart';
import '../services/cache_service.dart';

/// Game page - displays game details with background, news, and action buttons
class GamePage extends StatelessWidget {
  final Game game;
  
  const GamePage({
    super.key,
    required this.game,
  });
  
  @override
  Widget build(BuildContext context) {
    final provider = context.watch<AppProvider>();
    final content = provider.getContent(game.id);
    final isInstalled = provider.isGameInstalled(game.id);
    final progress = provider.getDownloadProgress(game.id);
    
    return Stack(
      fit: StackFit.expand,
      children: [
        // Background - video or image
        _GameBackground(display: game.display),
        
        // Content overlay - isolated with RepaintBoundary to prevent video texture artifacts
        Positioned.fill(
          child: RepaintBoundary(
            child: Padding(
              padding: const EdgeInsets.only(
                left: ElysiaTheme.sidebarWidth + 32,
                top: 32,
                right: 32,
                bottom: 32,
              ),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.end,
                children: [
                // Left side - News and Download control
                SizedBox(
                  width: 500,
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.end,
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      // News carousel
                      NewsWidget(content: content),
                      const SizedBox(height: 32),
                      
                      // Download/Launch control
                      DownloadControl(
                        gameId: game.id,
                        isInstalled: isInstalled,
                        progress: progress,
                        onActionPressed: () {
                          if (isInstalled) {
                            provider.launchGame(game.id);
                          } else {
                            provider.installGame(game.id);
                          }
                        },
                      ),
                    ],
                  ),
                ),
                
                const Spacer(),
                
                // Right side - Action buttons
                Column(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  crossAxisAlignment: CrossAxisAlignment.end,
                  children: [
                    // Top right button
                    GlassButton(
                      onPressed: () {
                        debugPrint('Explode button pressed!');
                      },
                      child: const Row(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          Icon(Icons.auto_awesome, size: 24),
                          SizedBox(width: 8),
                          Text('Explode', style: TextStyle(fontSize: 20)),
                        ],
                      ),
                    ),
                    
                    // Bottom right button
                    GlassButton(
                      onPressed: () {
                        debugPrint('Meow button pressed!');
                      },
                      child: const Row(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          Icon(Icons.pets, size: 24),
                          SizedBox(width: 8),
                          Text('Meow', style: TextStyle(fontSize: 20)),
                        ],
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ),
          ),
        ),
      ],
    );
  }
}

/// Combined background widget that handles both video and image backgrounds
class _GameBackground extends StatelessWidget {
  final Display display;
  
  const _GameBackground({required this.display});
  
  @override
  Widget build(BuildContext context) {
    // Use video background if available, otherwise fall back to image
    if (display.hasVideoBackground) {
      return _VideoBackground(
        key: ValueKey(display.videoBackgroundUrl),
        videoUrl: display.videoBackgroundUrl,
        themeImageUrl: display.themeImageUrl,
        fallbackImageUrl: display.background.url,
      );
    }
    
    return _BackgroundImage(url: display.background.url);
  }
}

/// Video background widget with optional theme image overlay
class _VideoBackground extends StatefulWidget {
  final String videoUrl;
  final String themeImageUrl;
  final String fallbackImageUrl;
  
  const _VideoBackground({
    super.key,
    required this.videoUrl,
    required this.themeImageUrl,
    required this.fallbackImageUrl,
  });
  
  @override
  State<_VideoBackground> createState() => _VideoBackgroundState();
}

class _VideoBackgroundState extends State<_VideoBackground> with WidgetsBindingObserver {
  Player? _player;
  VideoController? _videoController;
  bool _isInitialized = false;
  bool _hasError = false;
  bool _isDisposing = false;
  bool _showTheme = false;
  
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
    _initializeVideo();
  }
  
  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    if (_isDisposing) return;
    
    // Pause video when app is not visible to prevent crashes
    if (state == AppLifecycleState.paused || 
        state == AppLifecycleState.inactive ||
        state == AppLifecycleState.detached ||
        state == AppLifecycleState.hidden) {
      _pauseVideo();
    } else if (state == AppLifecycleState.resumed && _isInitialized) {
      _resumeVideo();
    }
  }
  
  void _pauseVideo() {
    if (_player != null && !_isDisposing && _player!.state.playing) {
      _player!.pause();
    }
  }
  
  void _resumeVideo() {
    if (!_isDisposing && _player != null && !_player!.state.playing) {
      _player!.play();
    }
  }
  
  Future<void> _initializeVideo() async {
    if (_isDisposing) return;
    
    try {
      _player = Player();
      _videoController = VideoController(_player!);
      
      await _player!.open(Media(widget.videoUrl));
      await _player!.setPlaylistMode(PlaylistMode.loop);
      await _player!.setVolume(0.0);
      
      if (_isDisposing || !mounted) {
        await _disposeResources();
        return;
      }
      
      if (mounted && !_isDisposing) {
        setState(() {
          _isInitialized = true;
        });
        
        // Show theme overlay after a short delay to ensure video has started rendering
        Future.delayed(const Duration(milliseconds: 300), () {
          if (mounted && !_isDisposing && _isInitialized) {
            setState(() {
              _showTheme = true;
            });
          }
        });
      }
    } catch (e) {
      debugPrint('Error initializing video: $e');
      if (mounted && !_isDisposing) {
        setState(() {
          _hasError = true;
        });
      }
    }
  }
  
  Future<void> _disposeResources() async {
    final player = _player;
    _player = null;
    _videoController = null;
    
    if (player != null) {
      try {
        await player.pause();
        await player.dispose();
      } catch (e) {
        debugPrint('Error disposing player: $e');
      }
    }
  }
  
  @override
  void dispose() {
    _isDisposing = true;
    _isInitialized = false;
    _showTheme = false;
    WidgetsBinding.instance.removeObserver(this);
    
    // Dispose resources asynchronously to prevent blocking
    Future.microtask(() async {
      await _disposeResources();
    });
    
    super.dispose();
  }
  
  @override
  Widget build(BuildContext context) {
    // Show fallback image if video failed to load
    if (_hasError) {
      return _BackgroundImage(url: widget.fallbackImageUrl);
    }
    
    // Show fallback while initializing
    if (!_isInitialized || _videoController == null) {
      return _BackgroundImage(url: widget.fallbackImageUrl);
    }
    
    return Padding(
      padding: EdgeInsets.only(left: ElysiaTheme.sidebarWidth),
      child: Stack(
        fit: StackFit.expand,
        children: [
          // Video player layer
          Video(controller: _videoController!),
          
          // Theme image overlay layer
          // Show when theme should be visible and theme URL exists
          if (widget.themeImageUrl.isNotEmpty && _showTheme)
            CachedNetworkImage(
              key: ValueKey('theme_${widget.themeImageUrl}'),
              imageUrl: widget.themeImageUrl,
              cacheManager: ElysiaCacheManager.instance,
              fit: BoxFit.cover,
              width: double.infinity,
              height: double.infinity,
              fadeInDuration: const Duration(milliseconds: 200),
              placeholder: (context, url) => const SizedBox.shrink(),
              errorWidget: (context, url, error) => const SizedBox.shrink(),
            ),
        ],
      ),
    );
  }
}

class _BackgroundImage extends StatelessWidget {
  final String url;
  
  const _BackgroundImage({required this.url});
  
  @override
  Widget build(BuildContext context) {
    if (url.isEmpty) {
      return Padding(
        padding: EdgeInsets.only(left: ElysiaTheme.sidebarWidth),
        child: Container(
          decoration: const BoxDecoration(
            gradient: LinearGradient(
              begin: Alignment.topLeft,
              end: Alignment.bottomRight,
              colors: [
                Color(0xFF1A1A2E),
                Color(0xFF16213E),
                Color(0xFF0F3460),
              ],
            ),
          ),
        ),
      );
    }
    
    return Padding(
      padding: EdgeInsets.only(left: ElysiaTheme.sidebarWidth),
      child: CachedNetworkImage(
        imageUrl: url,
        cacheManager: ElysiaCacheManager.instance,
        fit: BoxFit.cover,
        width: double.infinity,
        height: double.infinity,
        placeholder: (context, url) => Container(
          color: ElysiaTheme.backgroundColor,
        ),
        errorWidget: (context, url, error) => Container(
          color: ElysiaTheme.backgroundColor,
        ),
      ),
    );
  }
}
