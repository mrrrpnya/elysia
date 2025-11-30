import 'package:flutter/material.dart';
import '../theme/theme.dart';

/// Home page - settings and welcome screen
class HomePage extends StatelessWidget {
  const HomePage({super.key});
  
  @override
  Widget build(BuildContext context) {
    return Container(
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
      child: Padding(
        padding: EdgeInsets.only(
          left: ElysiaTheme.sidebarWidth + 32,
          top: 32,
          right: 32,
          bottom: 32,
        ),
        child: const Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            Icon(
              Icons.videogame_asset,
              size: 80,
              color: ElysiaTheme.primaryColor,
            ),
            SizedBox(height: 24),
            Text(
              'Welcome to Elysia',
              style: TextStyle(
                color: ElysiaTheme.textPrimary,
                fontSize: 32,
                fontWeight: FontWeight.bold,
              ),
            ),
            SizedBox(height: 8),
            Text(
              'Select a game from the sidebar to get started',
              style: TextStyle(
                color: ElysiaTheme.textSecondary,
                fontSize: 18,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

/// Settings page
class SettingsPage extends StatelessWidget {
  const SettingsPage({super.key});
  
  @override
  Widget build(BuildContext context) {
    return Container(
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
      child: Padding(
        padding: EdgeInsets.only(
          left: ElysiaTheme.sidebarWidth + 32,
          top: 32,
          right: 32,
          bottom: 32,
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Settings',
              style: TextStyle(
                color: ElysiaTheme.textPrimary,
                fontSize: 32,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 32),
            
            // Settings sections
            _SettingsSection(
              title: 'Directories',
              children: [
                _SettingsItem(
                  icon: Icons.folder,
                  title: 'Wine Prefixes Directory',
                  subtitle: '~/.local/share/elysia/wineprefixes',
                  onTap: () {
                    // TODO: Open directory picker
                  },
                ),
                _SettingsItem(
                  icon: Icons.folder,
                  title: 'Components Directory',
                  subtitle: '~/.local/share/elysia/components',
                  onTap: () {
                    // TODO: Open directory picker
                  },
                ),
                _SettingsItem(
                  icon: Icons.folder,
                  title: 'Cache Directory',
                  subtitle: '~/.local/share/elysia/cache',
                  onTap: () {
                    // TODO: Open directory picker
                  },
                ),
              ],
            ),
            
            const SizedBox(height: 24),
            
            _SettingsSection(
              title: 'About',
              children: [
                _SettingsItem(
                  icon: Icons.info_outline,
                  title: 'Version',
                  subtitle: '1.0.0',
                  onTap: null,
                ),
                _SettingsItem(
                  icon: Icons.code,
                  title: 'Source Code',
                  subtitle: 'github.com/mrrrpnya/elysia',
                  onTap: () {
                    // TODO: Open URL
                  },
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class _SettingsSection extends StatelessWidget {
  final String title;
  final List<Widget> children;
  
  const _SettingsSection({
    required this.title,
    required this.children,
  });
  
  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          title,
          style: const TextStyle(
            color: ElysiaTheme.textSecondary,
            fontSize: 14,
            fontWeight: FontWeight.w500,
          ),
        ),
        const SizedBox(height: 12),
        Container(
          decoration: BoxDecoration(
            color: ElysiaTheme.surfaceColor.withOpacity(0.5),
            borderRadius: BorderRadius.circular(ElysiaTheme.cardRadius),
            border: Border.all(color: ElysiaTheme.borderColor),
          ),
          child: Column(
            children: children,
          ),
        ),
      ],
    );
  }
}

class _SettingsItem extends StatefulWidget {
  final IconData icon;
  final String title;
  final String subtitle;
  final VoidCallback? onTap;
  
  const _SettingsItem({
    required this.icon,
    required this.title,
    required this.subtitle,
    this.onTap,
  });
  
  @override
  State<_SettingsItem> createState() => _SettingsItemState();
}

class _SettingsItemState extends State<_SettingsItem> {
  bool _isHovering = false;
  
  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) => setState(() => _isHovering = true),
      onExit: (_) => setState(() => _isHovering = false),
      cursor: widget.onTap != null 
          ? SystemMouseCursors.click 
          : SystemMouseCursors.basic,
      child: GestureDetector(
        onTap: widget.onTap,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 150),
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: _isHovering && widget.onTap != null
                ? ElysiaTheme.surfaceColor.withOpacity(0.3)
                : Colors.transparent,
          ),
          child: Row(
            children: [
              Icon(
                widget.icon,
                color: ElysiaTheme.textSecondary,
                size: 24,
              ),
              const SizedBox(width: 16),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      widget.title,
                      style: const TextStyle(
                        color: ElysiaTheme.textPrimary,
                        fontSize: 16,
                      ),
                    ),
                    const SizedBox(height: 2),
                    Text(
                      widget.subtitle,
                      style: const TextStyle(
                        color: ElysiaTheme.textSecondary,
                        fontSize: 12,
                      ),
                    ),
                  ],
                ),
              ),
              if (widget.onTap != null)
                const Icon(
                  Icons.chevron_right,
                  color: ElysiaTheme.textSecondary,
                ),
            ],
          ),
        ),
      ),
    );
  }
}
