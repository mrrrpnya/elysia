import 'dart:ui';
import 'package:flutter/material.dart';
import '../theme/theme.dart';
import 'package:elysia/api.dart' as api;
import '../extensions/api_extensions.dart';

/// Download control widget showing progress and action button
class DownloadControl extends StatelessWidget {
  final String gameId;
  final bool isInstalled;
  final api.DownloadProgress? progress;
  final VoidCallback? onActionPressed;
  
  const DownloadControl({
    super.key,
    required this.gameId,
    required this.isInstalled,
    this.progress,
    this.onActionPressed,
  });
  
  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        // Progress bar (if downloading)
        if (progress != null && progress!.isBusy) ...[
          _buildProgressBar(progress!),
          const SizedBox(height: 8),
        ],
        
        // Action button (if not busy)
        if (progress == null || !progress!.isBusy)
          _buildActionButton(),
      ],
    );
  }
  
  Widget _buildProgressBar(api.DownloadProgress progress) {
    final percentage = progress.percentage;
    final statusText = _buildStatusText(progress);
    
    return ClipRRect(
      borderRadius: BorderRadius.circular(ElysiaTheme.itemRadius),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 16, sigmaY: 16),
        child: Container(
          padding: const EdgeInsets.all(12),
          decoration: BoxDecoration(
            color: Colors.black.withValues(alpha: 0.35),
            borderRadius: BorderRadius.circular(ElysiaTheme.itemRadius),
            border: Border.all(color: ElysiaTheme.borderColor),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              // Progress bar
              Container(
                height: 24,
                decoration: BoxDecoration(
                  color: ElysiaTheme.borderColor,
                  borderRadius: BorderRadius.circular(4),
                ),
                child: FractionallySizedBox(
                  alignment: Alignment.centerLeft,
                  widthFactor: percentage / 100,
                  child: Container(
                    decoration: BoxDecoration(
                      color: ElysiaTheme.primaryColor,
                      borderRadius: BorderRadius.circular(4),
                    ),
                  ),
                ),
              ),
              
              const SizedBox(height: 6),
              
              // Status text
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Expanded(
                    child: Text(
                      statusText,
                      style: const TextStyle(
                        color: ElysiaTheme.textPrimary,
                        fontSize: 12,
                      ),
                      overflow: TextOverflow.ellipsis,
                    ),
                  ),
                  Text(
                    '${percentage.toStringAsFixed(1)}%',
                    style: const TextStyle(
                      color: ElysiaTheme.textPrimary,
                      fontSize: 12,
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
  
  String _buildStatusText(api.DownloadProgress progress) {
    if (progress.total > 0 && 
        (progress.status.startsWith('Downloading') || 
         progress.status.startsWith('Extracting'))) {
      if (progress.mbPerSecond > 0) {
        return '${progress.status} - ${progress.downloadedGb} GB / ${progress.totalGb} GB - ${progress.mbPerSecond.toStringAsFixed(2)} MB/s';
      } else {
        return '${progress.status} - ${progress.downloadedGb} GB / ${progress.totalGb} GB';
      }
    }
    return progress.status;
  }
  
  Widget _buildActionButton() {
    final buttonText = isInstalled ? 'Start Game' : 'Download Game';
    
    return _ActionButton(
      text: buttonText,
      onPressed: onActionPressed,
    );
  }
}

class _ActionButton extends StatefulWidget {
  final String text;
  final VoidCallback? onPressed;
  
  const _ActionButton({
    required this.text,
    this.onPressed,
  });
  
  @override
  State<_ActionButton> createState() => _ActionButtonState();
}

class _ActionButtonState extends State<_ActionButton> {
  bool _isHovering = false;
  
  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) => setState(() => _isHovering = true),
      onExit: (_) => setState(() => _isHovering = false),
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
        onTap: widget.onPressed,
        child: ClipRRect(
          borderRadius: BorderRadius.circular(ElysiaTheme.buttonRadius),
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 16, sigmaY: 16),
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 150),
              padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
              decoration: BoxDecoration(
                color: _isHovering
                    ? ElysiaTheme.primaryColor.withValues(alpha: 0.8)
                    : ElysiaTheme.primaryColor.withValues(alpha: 0.7),
                borderRadius: BorderRadius.circular(ElysiaTheme.buttonRadius),
                boxShadow: const [
                  BoxShadow(
                    color: ElysiaTheme.shadowColor,
                    blurRadius: 5,
                    offset: Offset(2, 2),
                  ),
                ],
              ),
              child: Text(
                widget.text,
                style: const TextStyle(
                  color: Colors.white,
                  fontSize: 24,
                  fontWeight: FontWeight.w600,
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
