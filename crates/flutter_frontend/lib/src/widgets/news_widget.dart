import 'package:flutter/material.dart' hide Banner;
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
  
  List<Banner> get banners => widget.content?.banners ?? [];
  
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
                // This would be implemented with platform-specific code
              },
              child: PageView.builder(
                itemCount: banners.length,
                onPageChanged: (index) {
                  setState(() => _currentIndex = index);
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
        
        // Page indicators
        if (banners.length > 1) ...[
          const SizedBox(height: 8),
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: List.generate(
              banners.length,
              (index) => _PageIndicator(
                isActive: index == _currentIndex,
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
    
    return CachedNetworkImage(
      imageUrl: imageUrl,
      fit: BoxFit.cover,
      placeholder: (context, url) => Container(
        color: ElysiaTheme.cardColor,
        child: const Center(
          child: CircularProgressIndicator(
            color: ElysiaTheme.primaryColor,
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
