import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:cached_network_image/cached_network_image.dart';
import 'src/theme/theme.dart';
import 'src/providers/app_provider.dart';
import 'src/pages/pages.dart';
import 'src/widgets/widgets.dart';
import 'src/services/cache_service.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Initialize the image cache manager
  await ElysiaCacheManager.initialize();

  runApp(const ElysiaApp());
}

/// Elysia Game Launcher - Flutter frontend with Rust backend
class ElysiaApp extends StatelessWidget {
  const ElysiaApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (_) => AppProvider()..initialize(),
      child: MaterialApp(
        title: 'Elysia',
        debugShowCheckedModeBanner: false,
        theme: ElysiaTheme.darkTheme,
        home: const AppShell(),
      ),
    );
  }
}

/// Main app shell with sidebar navigation
class AppShell extends StatefulWidget {
  const AppShell({super.key});

  @override
  State<AppShell> createState() => _AppShellState();
}

class _AppShellState extends State<AppShell> {
  bool _showSettings = false;
  bool _showComponents = false;

  @override
  Widget build(BuildContext context) {
    final provider = context.watch<AppProvider>();

    if (provider.isLoading) {
      return const Scaffold(
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              CircularProgressIndicator(color: ElysiaTheme.primaryColor),
              SizedBox(height: 16),
              Text(
                'Loading...',
                style: TextStyle(color: ElysiaTheme.textSecondary),
              ),
            ],
          ),
        ),
      );
    }

    if (provider.error != null) {
      return Scaffold(
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              const Icon(
                Icons.error_outline,
                color: Colors.red,
                size: 64,
              ),
              const SizedBox(height: 16),
              Text(
                'Error: ${provider.error}',
                style: const TextStyle(color: ElysiaTheme.textPrimary),
              ),
              const SizedBox(height: 16),
              ElevatedButton(
                onPressed: () => provider.initialize(),
                child: const Text('Retry'),
              ),
            ],
          ),
        ),
      );
    }

    return Scaffold(
      body: Stack(
        children: [
          // Main content
          AnimatedSwitcher(
            duration: const Duration(milliseconds: 300),
            child: _showComponents
                ? const ComponentsPage()
                : _showSettings
                    ? const SettingsPage()
                    : provider.selectedGame != null
                        ? GamePage(
                            key: ValueKey(provider.selectedGame!.id),
                            game: provider.selectedGame!,
                          )
                        : const HomePage(),
          ),

          // Sidebar
          _Sidebar(
            games: provider.games,
            selectedGame: provider.selectedGame,
            showSettings: _showSettings,
            showComponents: _showComponents,
            onGameSelected: (game) {
              setState(() {
                _showSettings = false;
                _showComponents = false;
              });
              provider.selectGame(game);
            },
            onComponentsPressed: () {
              setState(() {
                _showComponents = !_showComponents;
                if (_showComponents) _showSettings = false;
              });
            },
            onSettingsPressed: () {
              setState(() {
                _showSettings = !_showSettings;
                if (_showSettings) _showComponents = false;
              });
            },
          ),
        ],
      ),
    );
  }
}

/// Sidebar navigation widget
class _Sidebar extends StatelessWidget {
  final List games;
  final dynamic selectedGame;
  final bool showSettings;
  final bool showComponents;
  final Function(dynamic) onGameSelected;
  final VoidCallback onComponentsPressed;
  final VoidCallback onSettingsPressed;

  const _Sidebar({
    required this.games,
    required this.selectedGame,
    required this.showSettings,
    required this.showComponents,
    required this.onGameSelected,
    required this.onComponentsPressed,
    required this.onSettingsPressed,
  });

  @override
  Widget build(BuildContext context) {
    return Positioned(
      top: 0,
      left: 0,
      bottom: 0,
      width: ElysiaTheme.sidebarWidth,
      child: ClipRect(
        child: BackdropFilter(
          filter: ImageFilter.blur(sigmaX: 16, sigmaY: 16),
          child: Container(
            decoration: BoxDecoration(
              color: ElysiaTheme.sidebarColor,
              boxShadow: const [
                BoxShadow(
                  color: ElysiaTheme.shadowColor,
                  blurRadius: 5,
                  offset: Offset(2, 0),
                ),
              ],
            ),
            child: Column(
              children: [
                // Game list
                Expanded(
                  child: ListView.builder(
                    padding: const EdgeInsets.all(8),
                    itemCount: games.length,
                    itemBuilder: (context, index) {
                      final game = games[index];
                      final isSelected = selectedGame?.id == game.id &&
                          !showSettings &&
                          !showComponents;

                      return Padding(
                        padding: const EdgeInsets.only(bottom: 8),
                        child: SidebarItem(
                          isActive: isSelected,
                          onTap: () => onGameSelected(game),
                          child: _GameIcon(
                            iconUrl: game.iconUrl,
                            name: game.name,
                          ),
                        ),
                      );
                    },
                  ),
                ),

                // Components button (wine bottle icon)
                Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 8),
                  child: SidebarItem(
                    isActive: showComponents,
                    onTap: onComponentsPressed,
                    child: const Icon(
                      Icons.wine_bar,
                      size: 32,
                      color: ElysiaTheme.textPrimary,
                    ),
                  ),
                ),

                const SizedBox(height: 8),

                // Settings button
                Padding(
                  padding: const EdgeInsets.all(8),
                  child: SidebarItem(
                    isActive: showSettings,
                    onTap: onSettingsPressed,
                    child: const Icon(
                      Icons.settings,
                      size: 32,
                      color: ElysiaTheme.textPrimary,
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

/// Game icon widget for sidebar
class _GameIcon extends StatelessWidget {
  final String iconUrl;
  final String name;

  const _GameIcon({
    required this.iconUrl,
    required this.name,
  });

  @override
  Widget build(BuildContext context) {
    if (iconUrl.isEmpty) {
      return Container(
        width: 48,
        height: 48,
        decoration: BoxDecoration(
          color: ElysiaTheme.cardColor,
          borderRadius: BorderRadius.circular(8),
        ),
        child: Center(
          child: Text(
            name.isNotEmpty ? name[0].toUpperCase() : '?',
            style: const TextStyle(
              color: ElysiaTheme.textPrimary,
              fontSize: 24,
              fontWeight: FontWeight.bold,
            ),
          ),
        ),
      );
    }

    return ClipRRect(
      borderRadius: BorderRadius.circular(8),
      child: CachedNetworkImage(
        imageUrl: iconUrl,
        cacheManager: ElysiaCacheManager.instance,
        width: 48,
        height: 48,
        fit: BoxFit.cover,
        placeholder: (context, url) => Container(
          width: 48,
          height: 48,
          color: ElysiaTheme.cardColor,
          child: const Center(
            child: SizedBox(
              width: 24,
              height: 24,
              child: CircularProgressIndicator(
                strokeWidth: 2,
                color: ElysiaTheme.primaryColor,
              ),
            ),
          ),
        ),
        errorWidget: (context, url, error) => Container(
          width: 48,
          height: 48,
          color: ElysiaTheme.cardColor,
          child: Center(
            child: Text(
              name.isNotEmpty ? name[0].toUpperCase() : '?',
              style: const TextStyle(
                color: ElysiaTheme.textPrimary,
                fontSize: 24,
                fontWeight: FontWeight.bold,
              ),
            ),
          ),
        ),
      ),
    );
  }
}
