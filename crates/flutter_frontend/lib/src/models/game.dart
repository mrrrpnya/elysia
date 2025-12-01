/// Game model class - mirrors Rust Game struct
class Game {
  final String id;
  final String biz;
  final Display display;
  final String displayStatus;
  
  const Game({
    required this.id,
    required this.biz,
    required this.display,
    required this.displayStatus,
  });
  
  factory Game.fromJson(Map<String, dynamic> json) {
    return Game(
      id: json['id'] as String,
      biz: json['biz'] as String,
      display: Display.fromJson(json['display'] as Map<String, dynamic>),
      displayStatus: json['display_status'] as String,
    );
  }
}

/// Display model class
class Display {
  final String language;
  final String name;
  final Image icon;
  final String title;
  final String subtitle;
  final ImageLink background;
  final ImageLink logo;
  final ImageLink thumbnail;
  final Image shortcut;
  
  const Display({
    required this.language,
    required this.name,
    required this.icon,
    required this.title,
    required this.subtitle,
    required this.background,
    required this.logo,
    required this.thumbnail,
    required this.shortcut,
  });
  
  factory Display.fromJson(Map<String, dynamic> json) {
    return Display(
      language: json['language'] as String,
      name: json['name'] as String,
      icon: Image.fromJson(json['icon'] as Map<String, dynamic>),
      title: json['title'] as String,
      subtitle: json['subtitle'] as String? ?? '',
      background: ImageLink.fromJson(json['background'] as Map<String, dynamic>),
      logo: ImageLink.fromJson(json['logo'] as Map<String, dynamic>),
      thumbnail: ImageLink.fromJson(json['thumbnail'] as Map<String, dynamic>),
      shortcut: Image.fromJson(json['shortcut'] as Map<String, dynamic>),
    );
  }
}

/// Image model class
class Image {
  final String url;
  final String hoverUrl;
  final String link;
  final String md5;
  final int size;
  
  const Image({
    required this.url,
    required this.hoverUrl,
    required this.link,
    required this.md5,
    required this.size,
  });
  
  factory Image.fromJson(Map<String, dynamic> json) {
    return Image(
      url: json['url'] as String,
      hoverUrl: json['hover_url'] as String? ?? '',
      link: json['link'] as String? ?? '',
      md5: json['md5'] as String? ?? '',
      size: json['size'] as int? ?? 0,
    );
  }
}

/// ImageLink model class
class ImageLink {
  final String url;
  final String link;
  
  const ImageLink({
    required this.url,
    required this.link,
  });
  
  factory ImageLink.fromJson(Map<String, dynamic> json) {
    return ImageLink(
      url: json['url'] as String,
      link: json['link'] as String? ?? '',
    );
  }
}

/// Content model class - game content including banners and news
class Content {
  final GameInfo game;
  final String language;
  final List<Banner> banners;
  final List<Post> posts;
  final List<SocialMedia> socialMediaList;
  
  const Content({
    required this.game,
    required this.language,
    required this.banners,
    required this.posts,
    required this.socialMediaList,
  });
  
  factory Content.fromJson(Map<String, dynamic> json) {
    return Content(
      game: GameInfo.fromJson(json['game'] as Map<String, dynamic>),
      language: json['language'] as String,
      banners: (json['banners'] as List<dynamic>)
          .map((e) => Banner.fromJson(e as Map<String, dynamic>))
          .toList(),
      posts: (json['posts'] as List<dynamic>)
          .map((e) => Post.fromJson(e as Map<String, dynamic>))
          .toList(),
      socialMediaList: (json['social_media_list'] as List<dynamic>)
          .map((e) => SocialMedia.fromJson(e as Map<String, dynamic>))
          .toList(),
    );
  }
}

/// GameInfo model class
class GameInfo {
  final String id;
  final String biz;
  
  const GameInfo({
    required this.id,
    required this.biz,
  });
  
  factory GameInfo.fromJson(Map<String, dynamic> json) {
    return GameInfo(
      id: json['id'] as String,
      biz: json['biz'] as String,
    );
  }
}

/// Banner model class
class Banner {
  final String id;
  final ImageLink image;
  final String i18nIdentifier;
  
  const Banner({
    required this.id,
    required this.image,
    required this.i18nIdentifier,
  });
  
  factory Banner.fromJson(Map<String, dynamic> json) {
    return Banner(
      id: json['id'] as String,
      image: ImageLink.fromJson(json['image'] as Map<String, dynamic>),
      i18nIdentifier: json['i18n_identifier'] as String? ?? '',
    );
  }
}

/// Post model class
class Post {
  final String id;
  final String postType;
  final String title;
  final String link;
  final String date;
  
  const Post({
    required this.id,
    required this.postType,
    required this.title,
    required this.link,
    required this.date,
  });
  
  factory Post.fromJson(Map<String, dynamic> json) {
    return Post(
      id: json['id'] as String,
      postType: json['type'] as String,
      title: json['title'] as String,
      link: json['link'] as String,
      date: json['date'] as String,
    );
  }
}

/// SocialMedia model class
class SocialMedia {
  final String id;
  final Image icon;
  final String qrDesc;
  
  const SocialMedia({
    required this.id,
    required this.icon,
    required this.qrDesc,
  });
  
  factory SocialMedia.fromJson(Map<String, dynamic> json) {
    return SocialMedia(
      id: json['id'] as String,
      icon: Image.fromJson(json['icon'] as Map<String, dynamic>),
      qrDesc: json['qr_desc'] as String? ?? '',
    );
  }
}

/// Download progress model
class DownloadProgress {
  final int downloaded;
  final int total;
  final double mbPerSecond;
  final int partIndex;
  final int partsTotal;
  final String status;
  final bool isBusy;
  
  const DownloadProgress({
    required this.downloaded,
    required this.total,
    required this.mbPerSecond,
    required this.partIndex,
    required this.partsTotal,
    required this.status,
    required this.isBusy,
  });
  
  double get percentage => total > 0 ? (downloaded / total) * 100 : 0;
  
  String get downloadedGb => (downloaded / 1000000000).toStringAsFixed(2);
  String get totalGb => (total / 1000000000).toStringAsFixed(2);
}
