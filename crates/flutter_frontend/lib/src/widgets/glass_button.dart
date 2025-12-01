import 'package:flutter/material.dart';
import '../theme/theme.dart';

/// Glass-style button inspired by Collapse launcher
/// Note: BackdropFilter removed to prevent rendering issues with video backgrounds
class GlassButton extends StatefulWidget {
  final Widget child;
  final VoidCallback? onPressed;
  final bool enabled;
  final double? width;
  
  const GlassButton({
    super.key,
    required this.child,
    this.onPressed,
    this.enabled = true,
    this.width,
  });
  
  @override
  State<GlassButton> createState() => _GlassButtonState();
}

class _GlassButtonState extends State<GlassButton> {
  bool _isHovering = false;
  bool _isPressed = false;
  
  @override
  Widget build(BuildContext context) {
    // Use a more opaque background to simulate glass effect without BackdropFilter
    // BackdropFilter causes rendering issues with video backgrounds
    final backgroundColor = !widget.enabled
        ? ElysiaTheme.surfaceColor.withValues(alpha: 0.5)
        : _isPressed
            ? ElysiaTheme.surfaceColor.withValues(alpha: 0.9)
            : _isHovering
                ? ElysiaTheme.surfaceColor.withValues(alpha: 0.85)
                : ElysiaTheme.surfaceColor.withValues(alpha: 0.8);
    
    return MouseRegion(
      onEnter: (_) => setState(() => _isHovering = true),
      onExit: (_) => setState(() => _isHovering = false),
      cursor: widget.enabled ? SystemMouseCursors.click : SystemMouseCursors.basic,
      child: GestureDetector(
        onTapDown: (_) => setState(() => _isPressed = true),
        onTapUp: (_) => setState(() => _isPressed = false),
        onTapCancel: () => setState(() => _isPressed = false),
        onTap: widget.enabled ? widget.onPressed : null,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 150),
          width: widget.width,
          padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
          decoration: BoxDecoration(
            color: backgroundColor,
            borderRadius: BorderRadius.circular(ElysiaTheme.buttonRadius),
            border: Border.all(color: ElysiaTheme.borderColor),
            boxShadow: const [
              BoxShadow(
                color: ElysiaTheme.shadowColor,
                blurRadius: 5,
                offset: Offset(2, 2),
              ),
            ],
          ),
          child: DefaultTextStyle(
            style: TextStyle(
              color: widget.enabled 
                  ? ElysiaTheme.textPrimary 
                  : ElysiaTheme.textSecondary,
              fontSize: 16,
              fontWeight: FontWeight.w500,
            ),
            child: widget.child,
          ),
        ),
      ),
    );
  }
}

/// Action button with accent color - for primary actions like Download/Launch
class AccentButton extends StatefulWidget {
  final Widget child;
  final VoidCallback? onPressed;
  final bool enabled;
  final double? width;
  
  const AccentButton({
    super.key,
    required this.child,
    this.onPressed,
    this.enabled = true,
    this.width,
  });
  
  @override
  State<AccentButton> createState() => _AccentButtonState();
}

class _AccentButtonState extends State<AccentButton> {
  bool _isHovering = false;
  bool _isPressed = false;
  
  @override
  Widget build(BuildContext context) {
    final baseColor = ElysiaTheme.primaryColor;
    final backgroundColor = !widget.enabled
        ? baseColor.withValues(alpha: 0.3)
        : _isPressed
            ? baseColor.withValues(alpha: 0.9)
            : _isHovering
                ? baseColor.withValues(alpha: 0.8)
                : baseColor.withValues(alpha: 0.7);
    
    return MouseRegion(
      onEnter: (_) => setState(() => _isHovering = true),
      onExit: (_) => setState(() => _isHovering = false),
      cursor: widget.enabled ? SystemMouseCursors.click : SystemMouseCursors.basic,
      child: GestureDetector(
        onTapDown: (_) => setState(() => _isPressed = true),
        onTapUp: (_) => setState(() => _isPressed = false),
        onTapCancel: () => setState(() => _isPressed = false),
        onTap: widget.enabled ? widget.onPressed : null,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 150),
          width: widget.width,
          padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
          decoration: BoxDecoration(
            color: backgroundColor,
            borderRadius: BorderRadius.circular(ElysiaTheme.buttonRadius),
            boxShadow: const [
              BoxShadow(
                color: ElysiaTheme.shadowColor,
                blurRadius: 5,
                offset: Offset(2, 2),
              ),
            ],
          ),
          child: DefaultTextStyle(
            style: TextStyle(
              color: widget.enabled ? Colors.white : Colors.white60,
              fontSize: 16,
              fontWeight: FontWeight.w600,
            ),
            child: widget.child,
          ),
        ),
      ),
    );
  }
}
