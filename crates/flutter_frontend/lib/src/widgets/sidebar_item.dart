import 'package:flutter/material.dart';
import '../theme/theme.dart';

/// Sidebar item widget - displays game icon in the sidebar
class SidebarItem extends StatefulWidget {
  final Widget child;
  final bool isActive;
  final VoidCallback? onTap;

  const SidebarItem({
    super.key,
    required this.child,
    this.isActive = false,
    this.onTap,
  });

  @override
  State<SidebarItem> createState() => _SidebarItemState();
}

class _SidebarItemState extends State<SidebarItem> {
  bool _isHovering = false;

  @override
  Widget build(BuildContext context) {
    final backgroundColor = widget.isActive || _isHovering
        ? ElysiaTheme.surfaceColor.withValues(alpha: 0.4)
        : ElysiaTheme.surfaceColor.withValues(alpha: 0.1);

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovering = true),
      onExit: (_) => setState(() => _isHovering = false),
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
        onTap: widget.onTap,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 150),
          padding: const EdgeInsets.all(8),
          decoration: BoxDecoration(
            color: backgroundColor,
            borderRadius: BorderRadius.circular(ElysiaTheme.itemRadius),
          ),
          child: widget.child,
        ),
      ),
    );
  }
}
