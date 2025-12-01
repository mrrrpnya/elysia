import 'dart:async';
import 'package:flutter/material.dart' hide Banner;
import 'package:flutter/gestures.dart';
import 'package:cached_network_image/cached_network_image.dart';
import '../models/models.dart';
import '../theme/theme.dart';

/// News/Banner widget with carousel
class NewsWidget extends StatefulWidget {
  final Content? content;
  
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
  
  List<Banner> get banners => widget.content?.banners ?? [];
  
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
        // Banner carousel
        ClipRRect(
          borderRadius: BorderRadius.circular(ElysiaTheme.cardRadius),
          child: AspectRatio(
            aspectRatio: 16 / 9,
            child: Listener(
              onPointerSignal: (event) {
                // Handle mouse wheel for carousel
                if (event is PointerScrollEvent && banners.length > 1) {
                  if (event.scrollDelta.dy > 0) {
                    // Scroll down - next (wrap to first)
                    final nextPage = (_currentIndex + 1) % banners.length;
                    _goToPage(nextPage);
                  } else if (event.scrollDelta.dy < 0) {
                    // Scroll up - previous (wrap to last)
                    final prevPage = (_currentIndex - 1 + banners.length) % banners.length;
                    _goToPage(prevPage);
                  }
                }
              },
              child: GestureDetector(
                onTap: _resetAutoScroll,
                child: PageView.builder(
                  controller: _pageController,
                  itemCount: banners.length,
                  onPageChanged: (index) {
                    setState(() => _currentIndex = index);
                    _resetAutoScroll();
                  },
                  itemBuilder: (context, index) {
                    final banner = banners[index];
                    return _BannerImage(
                      imageUrl: banner.image.url,
                    );
                  },
                ),
              ),
            ),
          ),
        ),
        
        // Page indicators
        if (banners.length > 1) ...[
          const SizedBox(height: 8),
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: List.generate(
              banners.length,
              (index) => GestureDetector(
                onTap: () => _goToPage(index),
                child: _PageIndicator(
                  isActive: index == _currentIndex,
                ),
              ),
            ),
          ),
        ],
      ],
    );
  }
}

class _BannerImage extends StatelessWidget {
  final String imageUrl;
  
  const _BannerImage({
    required this.imageUrl,
  });
  
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
    
    return Container(
      color: ElysiaTheme.cardColor,
      child: CachedNetworkImage(
        imageUrl: imageUrl,
        fit: BoxFit.contain,
        alignment: Alignment.center,
        placeholder: (context, url) => const Center(
          child: CircularProgressIndicator(
            color: ElysiaTheme.primaryColor,
          ),
        ),
        errorWidget: (context, url, error) => const Center(
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
  
  const _PageIndicator({
    required this.isActive,
  });
  
  @override
  Widget build(BuildContext context) {
    return AnimatedContainer(
      duration: const Duration(milliseconds: 200),
      margin: const EdgeInsets.symmetric(horizontal: 4),
      width: isActive ? 24 : 8,
      height: 8,
      decoration: BoxDecoration(
        color: isActive ? ElysiaTheme.primaryColor : ElysiaTheme.borderColor,
        borderRadius: BorderRadius.circular(4),
      ),
    );
  }
}
