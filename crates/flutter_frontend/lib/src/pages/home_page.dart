import 'package:flutter/material.dart';
import '../theme/theme.dart';

/// Home page - welcome screen
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
