import 'package:flutter/material.dart';

/// Elysia app theme - inspired by Collapse launcher and the original Freya design
class ElysiaTheme {
  // Colors
  static const Color primaryColor = Color(0xFFFF9500);
  static const Color accentColor = Color(0xFFFFB347);
  static const Color backgroundColor = Color(0xFF1A1A1A);
  static const Color surfaceColor = Color(0xFF222222);
  static const Color cardColor = Color(0xFF2A2A2A);
  static const Color sidebarColor = Color(0x66222222);
  static const Color textPrimary = Color(0xFFFFFFFF);
  static const Color textSecondary = Color(0xFFB3B3B3);
  static const Color borderColor = Color(0xFF404040);
  static const Color shadowColor = Color(0x4D000000);
  
  // Sizing
  static const double sidebarWidth = 84.0;
  static const double buttonRadius = 99.0;
  static const double cardRadius = 16.0;
  static const double itemRadius = 8.0;
  
  // Padding
  static const EdgeInsets pagePadding = EdgeInsets.all(32.0);
  static const EdgeInsets cardPadding = EdgeInsets.all(12.0);
  
  static ThemeData get darkTheme {
    return ThemeData(
      useMaterial3: true,
      brightness: Brightness.dark,
      colorScheme: ColorScheme.dark(
        primary: primaryColor,
        secondary: accentColor,
        surface: surfaceColor,
        onSurface: textPrimary,
        onPrimary: Colors.black,
      ),
      scaffoldBackgroundColor: backgroundColor,
      cardColor: cardColor,
      dividerColor: borderColor,
      
      // AppBar
      appBarTheme: const AppBarTheme(
        backgroundColor: Colors.transparent,
        elevation: 0,
        titleTextStyle: TextStyle(
          color: textPrimary,
          fontSize: 20,
          fontWeight: FontWeight.w600,
        ),
      ),
      
      // Text
      textTheme: const TextTheme(
        displayLarge: TextStyle(color: textPrimary, fontSize: 32, fontWeight: FontWeight.bold),
        displayMedium: TextStyle(color: textPrimary, fontSize: 28, fontWeight: FontWeight.bold),
        displaySmall: TextStyle(color: textPrimary, fontSize: 24, fontWeight: FontWeight.bold),
        headlineMedium: TextStyle(color: textPrimary, fontSize: 20, fontWeight: FontWeight.w600),
        titleLarge: TextStyle(color: textPrimary, fontSize: 18, fontWeight: FontWeight.w500),
        titleMedium: TextStyle(color: textPrimary, fontSize: 16, fontWeight: FontWeight.w500),
        bodyLarge: TextStyle(color: textPrimary, fontSize: 16),
        bodyMedium: TextStyle(color: textSecondary, fontSize: 14),
        labelLarge: TextStyle(color: textPrimary, fontSize: 14, fontWeight: FontWeight.w500),
      ),
      
      // Buttons
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: surfaceColor.withValues(alpha: 0.6),
          foregroundColor: textPrimary,
          padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(buttonRadius),
            side: const BorderSide(color: borderColor),
          ),
          elevation: 0,
        ),
      ),
      
      // Cards
      cardTheme: CardThemeData(
        color: cardColor,
        elevation: 0,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.all(Radius.circular(cardRadius)),
          side: BorderSide(color: borderColor),
        ),
      ),
      
      // Icons
      iconTheme: const IconThemeData(
        color: textPrimary,
        size: 24,
      ),
      
      // Progress indicators
      progressIndicatorTheme: const ProgressIndicatorThemeData(
        color: primaryColor,
        linearTrackColor: borderColor,
      ),
    );
  }
}

/// Glass effect decoration
class GlassDecoration extends BoxDecoration {
  GlassDecoration({
    Color? color,
    double borderRadius = ElysiaTheme.cardRadius,
  }) : super(
    color: color ?? ElysiaTheme.surfaceColor.withValues(alpha: 0.4),
    borderRadius: BorderRadius.circular(borderRadius),
    border: Border.all(color: ElysiaTheme.borderColor),
    boxShadow: const [
      BoxShadow(
        color: ElysiaTheme.shadowColor,
        blurRadius: 8,
        offset: Offset(2, 2),
      ),
    ],
  );
}
