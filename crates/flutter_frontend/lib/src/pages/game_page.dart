import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:cached_network_image/cached_network_image.dart';
import 'package:video_player/video_player.dart';
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
        
        // Content overlay
        Positioned.fill(
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
  VideoPlayerController? _controller;
  bool _isInitialized = false;
  bool _hasError = false;
  bool _isDisposing = false;
  
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
    if (_controller != null && _controller!.value.isInitialized) {
      _controller!.pause();
    }
  }
  
  void _resumeVideo() {
    if (!_isDisposing && _controller != null && _controller!.value.isInitialized) {
      _controller!.play();
    }
  }
  
  Future<void> _initializeVideo() async {
    if (_isDisposing) return;
    
    try {
      _controller = VideoPlayerController.networkUrl(
        Uri.parse(widget.videoUrl),
      );
      
      await _controller!.initialize();
      
      if (_isDisposing || !mounted) {
        await _disposeController();
        return;
      }
      
      await _controller!.setLooping(true);
      await _controller!.setVolume(0); // Mute the video background
      await _controller!.play();
      
      if (mounted && !_isDisposing) {
        setState(() {
          _isInitialized = true;
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
  
  Future<void> _disposeController() async {
    final controller = _controller;
    _controller = null;
    
    if (controller != null) {
      try {
        if (controller.value.isInitialized) {
          await controller.pause();
          // Small delay to allow the video to fully pause before dispose
          await Future.delayed(const Duration(milliseconds: 100));
        }
        await controller.dispose();
      } catch (e) {
        debugPrint('Error disposing video controller: $e');
      }
    }
  }
  
  @override
  void dispose() {
    _isDisposing = true;
    _isInitialized = false;
    WidgetsBinding.instance.removeObserver(this);
    
    // Capture controller reference before nullifying
    final controller = _controller;
    _controller = null;
    
    // Pause immediately if possible
    if (controller != null) {
      try {
        controller.pause();
      } catch (e) {
        debugPrint('Error pausing video on dispose: $e');
      }
      
      // Schedule disposal for after the current frame to avoid race conditions
      // with the GPU texture being released while still in use
      WidgetsBinding.instance.addPostFrameCallback((_) {
        try {
          controller.dispose();
        } catch (e) {
          debugPrint('Error disposing video controller: $e');
        }
      });
    }
    
    super.dispose();
  }
  
  @override
  Widget build(BuildContext context) {
    // Show fallback image if video failed to load
    if (_hasError) {
      return _BackgroundImage(url: widget.fallbackImageUrl);
    }
    
    // Show fallback while initializing
    if (!_isInitialized || _controller == null) {
      return _BackgroundImage(url: widget.fallbackImageUrl);
    }
    
    // Get the video dimensions
    final videoWidth = _controller!.value.size.width;
    final videoHeight = _controller!.value.size.height;
    
    return Padding(
      padding: EdgeInsets.only(left: ElysiaTheme.sidebarWidth),
      child: Stack(
        fit: StackFit.expand,
        children: [
          // Video player
          FittedBox(
            fit: BoxFit.cover,
            child: SizedBox(
              width: videoWidth,
              height: videoHeight,
              child: VideoPlayer(_controller!),
            ),
          ),
          
          // Theme image overlay on top of video (if available)
          if (widget.themeImageUrl.isNotEmpty)
            CachedNetworkImage(
              imageUrl: widget.themeImageUrl,
              cacheManager: ElysiaCacheManager.instance,
              fit: BoxFit.cover,
              width: double.infinity,
              height: double.infinity,
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
