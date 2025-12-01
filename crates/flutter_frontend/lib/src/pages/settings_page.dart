import 'dart:io';
import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import '../theme/theme.dart';

/// Settings page
class SettingsPage extends StatelessWidget {
  const SettingsPage({super.key});
  
  String get _baseDir {
    final homeDir = Platform.environment['HOME'] ?? '/home';
    return '$homeDir/.local/share/elysia';
  }
  
  Future<void> _openDirectory(BuildContext context, String path) async {
    // Open the directory in the system file manager
    final uri = Uri.parse('file://$path');
    if (await canLaunchUrl(uri)) {
      await launchUrl(uri);
    } else {
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Could not open directory: $path')),
        );
      }
    }
  }
  
  Future<void> _openUrl(BuildContext context, String url) async {
    final uri = Uri.parse(url);
    if (await canLaunchUrl(uri)) {
      await launchUrl(uri, mode: LaunchMode.externalApplication);
    } else {
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Could not open URL: $url')),
        );
      }
    }
  }
  
  @override
  Widget build(BuildContext context) {
    final baseDir = _baseDir;
    
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
                  subtitle: '$baseDir/wineprefixes',
                  onTap: () => _openDirectory(context, '$baseDir/wineprefixes'),
                ),
                _SettingsItem(
                  icon: Icons.folder,
                  title: 'Components Directory',
                  subtitle: '$baseDir/components',
                  onTap: () => _openDirectory(context, '$baseDir/components'),
                ),
                _SettingsItem(
                  icon: Icons.folder,
                  title: 'Cache Directory',
                  subtitle: '$baseDir/cache',
                  onTap: () => _openDirectory(context, '$baseDir/cache'),
                ),
              ],
            ),
            
            const SizedBox(height: 24),
            
            _SettingsSection(
              title: 'About',
              children: [
                const _SettingsItem(
                  icon: Icons.info_outline,
                  title: 'Version',
                  subtitle: '1.0.0',
                  onTap: null,
                ),
                _SettingsItem(
                  icon: Icons.code,
                  title: 'Source Code',
                  subtitle: 'github.com/mrrrpnya/elysia',
                  onTap: () => _openUrl(context, 'https://github.com/mrrrpnya/elysia'),
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
            color: ElysiaTheme.surfaceColor.withValues(alpha: 0.5),
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
                ? ElysiaTheme.surfaceColor.withValues(alpha: 0.3)
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
