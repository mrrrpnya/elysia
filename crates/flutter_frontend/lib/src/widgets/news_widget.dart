import 'dart:async';
import 'package:flutter/material.dart' hide Banner;
import 'package:flutter/gestures.dart';
import 'package:cached_network_image/cached_network_image.dart';
import 'package:elysia/api.dart' as api;
import '../theme/theme.dart';
import '../services/cache_service.dart';

/// News/Banner widget with carousel
class NewsWidget extends StatefulWidget {
  final api.FfiContent? content;

  const NewsWidget({
    super.key,
    this.content,
  });

  @override
  State<NewsWidget> createState() => _NewsWidgetState();
}

class _NewsWidgetState extends State<NewsWidget> {
  int _currentIndex = 0;
  Timer? _autoScrollTimer;
  late PageController _pageController;

  List<api.FfiBanner> get banners => widget.content?.banners ?? [];

  @override
  void initState() {
    super.initState();
    _pageController = PageController();
    _startAutoScroll();
  }

  @override
  void dispose() {
    _autoScrollTimer?.cancel();
    _pageController.dispose();
    super.dispose();
  }

  void _startAutoScroll() {
    _autoScrollTimer?.cancel();
    if (banners.length > 1) {
      _autoScrollTimer = Timer.periodic(const Duration(seconds: 5), (_) {
        if (_pageController.hasClients) {
          final nextPage = (_currentIndex + 1) % banners.length;
          _pageController.animateToPage(
            nextPage,
            duration: const Duration(milliseconds: 400),
            curve: Curves.easeInOut,
          );
        }
      });
    }
  }

  void _resetAutoScroll() {
    _startAutoScroll();
  }

  void _goToPage(int page) {
    if (_pageController.hasClients) {
      _pageController.animateToPage(
        page,
        duration: const Duration(milliseconds: 300),
        curve: Curves.easeInOut,
      );
    }
    _resetAutoScroll();
  }

  @override
  Widget build(BuildContext context) {
    if (banners.isEmpty) {
      return const SizedBox.shrink();
    }

    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        // Banner carousel with intrinsic height based on image
        ClipRRect(
          borderRadius: BorderRadius.circular(ElysiaTheme.cardRadius),
          child: Stack(
            children: [
              // Image carousel
              Listener(
                onPointerSignal: (event) {
                  // Handle mouse wheel for carousel
                  if (event is PointerScrollEvent && banners.length > 1) {
                    if (event.scrollDelta.dy > 0) {
                      final nextPage = (_currentIndex + 1) % banners.length;
                      _goToPage(nextPage);
                    } else if (event.scrollDelta.dy < 0) {
                      final prevPage =
                          (_currentIndex - 1 + banners.length) % banners.length;
                      _goToPage(prevPage);
                    }
                  }
                },
                child: GestureDetector(
                  onTap: _resetAutoScroll,
                  child: SizedBox(
                    // Use a reasonable height that works for most banner aspect ratios
                    height: 200,
                    child: PageView.builder(
                      controller: _pageController,
                      itemCount: banners.length,
                      onPageChanged: (index) {
                        setState(() => _currentIndex = index);
                        _resetAutoScroll();
                      },
                      itemBuilder: (context, index) {
                        final banner = banners[index];
                        return _BannerImage(imageUrl: banner.imageUrl);
                      },
                    ),
                  ),
                ),
              ),

              // Page indicators overlay at bottom with background for visibility
              if (banners.length > 1)
                Positioned(
                  left: 0,
                  right: 0,
                  bottom: 8,
                  child: Center(
                    child: Container(
                      padding: const EdgeInsets.symmetric(
                          horizontal: 12, vertical: 6),
                      decoration: BoxDecoration(
                        color: Colors.black.withValues(alpha: 0.5),
                        borderRadius: BorderRadius.circular(16),
                      ),
                      child: Row(
                        mainAxisSize: MainAxisSize.min,
                        children: List.generate(
                          banners.length,
                          (index) => GestureDetector(
                            onTap: () => _goToPage(index),
                            child: _PageIndicator(
                                isActive: index == _currentIndex),
                          ),
                        ),
                      ),
                    ),
                  ),
                ),
            ],
          ),
        ),
      ],
    );
  }
}

class _BannerImage extends StatelessWidget {
  final String imageUrl;

  const _BannerImage({required this.imageUrl});

  @override
  Widget build(BuildContext context) {
    if (imageUrl.isEmpty) {
      return Container(
        color: ElysiaTheme.cardColor,
        child: const Center(
          child: Icon(
            Icons.image_not_supported,
            color: ElysiaTheme.textSecondary,
            size: 48,
          ),
        ),
      );
    }

    // Use BoxFit.contain to show full image without cropping
    // The image will fill the width and maintain aspect ratio
    return CachedNetworkImage(
      imageUrl: imageUrl,
      cacheManager: ElysiaCacheManager.instance,
      fit: BoxFit.contain,
      alignment: Alignment.center,
      placeholder: (context, url) => Container(
        color: ElysiaTheme.cardColor,
        child: const Center(
          child: SizedBox(
            width: 32,
            height: 32,
            child: CircularProgressIndicator(
              strokeWidth: 2,
              color: ElysiaTheme.primaryColor,
            ),
          ),
        ),
      ),
      errorWidget: (context, url, error) => Container(
        color: ElysiaTheme.cardColor,
        child: const Center(
          child: Icon(
            Icons.broken_image,
            color: ElysiaTheme.textSecondary,
            size: 48,
          ),
        ),
      ),
    );
  }
}

class _PageIndicator extends StatelessWidget {
  final bool isActive;

  const _PageIndicator({required this.isActive});

  @override
  Widget build(BuildContext context) {
    return AnimatedContainer(
      duration: const Duration(milliseconds: 200),
      margin: const EdgeInsets.symmetric(horizontal: 3),
      width: isActive ? 20 : 6,
      height: 6,
      decoration: BoxDecoration(
        color: isActive
            ? ElysiaTheme.primaryColor
            : Colors.white.withValues(alpha: 0.6),
        borderRadius: BorderRadius.circular(3),
      ),
    );
  }
}
