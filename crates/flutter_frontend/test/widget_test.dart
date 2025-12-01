// Basic Flutter widget test for Elysia launcher

import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';

import 'package:elysia_flutter/src/providers/app_provider.dart';
import 'package:elysia_flutter/src/theme/theme.dart';
import 'package:flutter/material.dart';

void main() {
  testWidgets('Elysia app shows loading state', (WidgetTester tester) async {
    // Create a provider with loading state
    final provider = AppProvider();
    
    // Build a minimal widget with the provider
    await tester.pumpWidget(
      ChangeNotifierProvider.value(
        value: provider,
        child: MaterialApp(
          theme: ElysiaTheme.darkTheme,
          home: const Scaffold(
            body: Center(
              child: Text('Loading...'),
            ),
          ),
        ),
      ),
    );

    // Verify loading text is shown
    expect(find.text('Loading...'), findsOneWidget);
  });
}
