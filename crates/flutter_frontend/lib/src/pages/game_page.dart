import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:cached_network_image/cached_network_image.dart';
import '../providers/app_provider.dart';
import '../theme/theme.dart';
import '../widgets/widgets.dart';
import '../services/cache_service.dart';
import 'package:elysia/ffi.dart' as api;
import 'dart:async';
import 'dart:ui' as ui;
import 'dart:typed_data';

/// Game page - displays game details with background, news, and action buttons
class GamePage extends StatelessWidget {
  final api.FfiGame game;

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
        _GameBackground(game: game),

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
  final api.FfiGame game;

  const _GameBackground({required this.game});

  @override
  Widget build(BuildContext context) {
    // Use video background if available, otherwise fall back to image
    if (game.hasVideoBackground) {
      return _VideoBackground(
        key: ValueKey(game.videoBackgroundUrl),
        videoUrl: game.videoBackgroundUrl,
        themeImageUrl: game.themeImageUrl,
        fallbackImageUrl: game.backgroundUrl,
      );
    }

    return _BackgroundImage(url: game.backgroundUrl);
  }
}

/// Video background widget with optional theme image overlay
/// Uses frame-by-frame rendering from Rust backend
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

class _VideoBackgroundState extends State<_VideoBackground> {
  StreamSubscription? _frameSubscription;
  api.FfiVideoFrame? _currentFrame;
  bool _isLoading = true;
  bool _hasError = false;
  Timer? _loopTimer;
  ui.Image? _currentImage;
  bool _isStartingStream = false;

  @override
  void initState() {
    super.initState();
    _startVideoStream();
  }

  @override
  void didUpdateWidget(_VideoBackground oldWidget) {
    super.didUpdateWidget(oldWidget);
    // Restart video stream if URL changed
    if (oldWidget.videoUrl != widget.videoUrl) {
      debugPrint(
          'Video URL changed from ${oldWidget.videoUrl} to ${widget.videoUrl}');
      // Don't start a new stream if one is already starting
      if (!_isStartingStream) {
        _startVideoStream();
      }
    }
  }

  void _startVideoStream() async {
    // Prevent concurrent stream starts
    if (_isStartingStream) {
      debugPrint('Stream start already in progress, ignoring request');
      return;
    }

    _isStartingStream = true;

    try {
      // Cancel any existing subscription first to stop the decoder
      debugPrint('Cancelling existing video stream (if any)');
      final oldSubscription = _frameSubscription;
      _frameSubscription = null;
      _loopTimer?.cancel();
      _loopTimer = null;

      // Dispose old image before clearing state
      try {
        _currentImage?.dispose();
        _currentImage = null;
      } catch (e) {
        debugPrint('Error disposing old image: $e');
      }

      // Cancel old subscription to close the channel
      await oldSubscription?.cancel();

      // Give the Rust decoder more time to detect the closed channel and clean up
      await Future.delayed(const Duration(milliseconds: 150));

      if (!mounted) {
        debugPrint('Widget unmounted during stream restart, aborting');
        _isStartingStream = false;
        return;
      }

      setState(() {
        _isLoading = true;
        _hasError = false;
        _currentFrame = null;
        _currentImage = null;
      });

      debugPrint('Starting video stream: ${widget.videoUrl}');

      // Get video frame stream from Rust
      final stream = api.stream_video_frames(url: widget.videoUrl);

      _frameSubscription = stream.listen(
        (frame) async {
          if (!mounted) {
            // Widget unmounted, just return without processing the frame
            return;
          }

          try {
            // Decode RGBA frame to ui.Image
            final buffer = await ui.ImmutableBuffer.fromUint8List(
                Uint8List.fromList(frame.data));
            final descriptor = ui.ImageDescriptor.raw(
              buffer,
              width: frame.width.toInt(),
              height: frame.height.toInt(),
              pixelFormat: ui.PixelFormat.rgba8888,
            );
            final codec = await descriptor.instantiateCodec();
            final frameInfo = await codec.getNextFrame();

            // Dispose codec/descriptor/buffer immediately after getting the image
            // This frees the temporary decoding memory (~8MB per frame)
            codec.dispose();
            descriptor.dispose();
            buffer.dispose();

            if (mounted) {
              // Store references that need to be disposed
              final oldImage = _currentImage;
              final oldFrame = _currentFrame;

              setState(() {
                _currentFrame = frame;
                _currentImage = frameInfo.image;
                _isLoading = false;
              });

              // Dispose old image after state update
              if (oldImage != null) {
                try {
                  oldImage.dispose();
                } catch (e) {
                  debugPrint('Error disposing old image: $e');
                }
              }
              // Clear old frame data reference to release memory
              // (the frame object itself will be garbage collected)
            } else {
              // Widget unmounted, dispose the new image
              try {
                frameInfo.image.dispose();
              } catch (e) {
                debugPrint('Error disposing unmounted image: $e');
              }
            }
          } catch (e) {
            debugPrint('Error decoding frame: $e');
          }
        },
        onError: (error) {
          debugPrint('Video stream error: $error');
          if (mounted) {
            setState(() {
              _hasError = true;
              _isLoading = false;
            });
          }
        },
        onDone: () {
          debugPrint('Video playback finished, looping...');
          // Video finished, loop by restarting
          if (mounted && !_hasError) {
            _loopTimer = Timer(const Duration(milliseconds: 100), () {
              if (mounted) {
                _startVideoStream();
              }
            });
          }
        },
        cancelOnError: false,
      );

      _isStartingStream = false;
      debugPrint('Video stream started successfully');
    } catch (e) {
      debugPrint('Error starting video stream: $e');
      _isStartingStream = false;
      if (mounted) {
        setState(() {
          _hasError = true;
          _isLoading = false;
        });
      }
    }
  }

  @override
  void dispose() {
    debugPrint('VideoBackground disposing - cancelling stream');
    // Cancel subscription immediately - this closes the stream sink and signals the Rust decoder to stop
    final subscription = _frameSubscription;
    _frameSubscription = null;
    _loopTimer?.cancel();
    _loopTimer = null;

    // Clear all references to help garbage collection
    _currentFrame = null;

    // Dispose image safely
    try {
      _currentImage?.dispose();
      _currentImage = null;
    } catch (e) {
      debugPrint('Error disposing image: $e');
    }

    super.dispose();

    // Cancel subscription after super.dispose() to ensure the channel is closed
    // This must happen last to properly signal the Rust decoder
    subscription?.cancel();
  }

  @override
  Widget build(BuildContext context) {
    // Show fallback image if video failed to load
    if (_hasError) {
      return _BackgroundImage(url: widget.fallbackImageUrl);
    }

    // Show fallback while loading first frame
    if (_isLoading || _currentImage == null) {
      return _BackgroundImage(url: widget.fallbackImageUrl);
    }

    return Padding(
      padding: EdgeInsets.only(left: ElysiaTheme.sidebarWidth),
      child: Stack(
        fit: StackFit.expand,
        children: [
          // Video frame layer - display current frame as image
          RawImage(
            image: _currentImage,
            fit: BoxFit.cover,
          ),

          // Theme image overlay layer
          if (widget.themeImageUrl.isNotEmpty)
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
