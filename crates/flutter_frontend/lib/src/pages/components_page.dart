import 'package:flutter/material.dart';
import '../theme/theme.dart';
import 'package:elysia/api.dart' as api;

/// Components page for managing wine/proton runners
class ComponentsPage extends StatefulWidget {
  const ComponentsPage({super.key});

  @override
  State<ComponentsPage> createState() => _ComponentsPageState();
}

class _ComponentsPageState extends State<ComponentsPage> {
  List<api.AvailableRunner> _runners = [];
  List<api.AvailableComponent> _components = [];
  final Map<String, bool> _isLoading = {};

  @override
  void initState() {
    super.initState();
    _loadData();
  }

  void _loadData() {
    _loadRunners();
    _loadComponents();
  }

  void _loadRunners() {
    try {
      // Use generated classes directly - no conversion!
      final runners = api.getAvailableRunners();
      setState(() {
        _runners = runners;
      });
    } catch (e) {
      debugPrint('Failed to load runners: $e');
    }
  }

  void _loadComponents() {
    try {
      // Use generated classes directly - no conversion!
      final components = api.getAvailableComponents();
      setState(() {
        _components = components;
      });
    } catch (e) {
      debugPrint('Failed to load components: $e');
    }
  }

  Future<void> _installRunner(String runnerName) async {
    setState(() {
      _isLoading[runnerName] = true;
    });

    try {
      final result = await api.installRunner(runnerName: runnerName);
      if (result == 'ok') {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text('$runnerName installed successfully'),
              backgroundColor: Colors.green,
            ),
          );
        }
        _loadRunners();
      } else {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text('Failed to install: $result'),
              backgroundColor: Colors.red,
            ),
          );
        }
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Failed to install: $e'),
            backgroundColor: Colors.red,
          ),
        );
      }
    } finally {
      setState(() {
        _isLoading[runnerName] = false;
      });
    }
  }

  Future<void> _deleteRunner(String runnerName) async {
    setState(() {
      _isLoading[runnerName] = true;
    });

    try {
      final result = await api.deleteRunner(runnerName: runnerName);
      if (result == 'ok') {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text('$runnerName removed')),
          );
        }
        _loadRunners();
      } else {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text('Failed to remove: $result'),
              backgroundColor: Colors.red,
            ),
          );
        }
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Failed to remove: $e'),
            backgroundColor: Colors.red,
          ),
        );
      }
    } finally {
      setState(() {
        _isLoading[runnerName] = false;
      });
    }
  }

  Future<void> _installComponent(String componentName) async {
    setState(() {
      _isLoading[componentName] = true;
    });

    try {
      String result;
      if (componentName == 'umu-launcher') {
        result = await api.installUmuLauncher();
      } else if (componentName == 'jadeite') {
        result = await api.installJadeite();
      } else {
        result = 'Unknown component';
      }

      if (result == 'ok') {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text('$componentName installed successfully'),
              backgroundColor: Colors.green,
            ),
          );
        }
        _loadComponents();
      } else {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text('Failed to install: $result'),
              backgroundColor: Colors.red,
            ),
          );
        }
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Failed to install: $e'),
            backgroundColor: Colors.red,
          ),
        );
      }
    } finally {
      setState(() {
        _isLoading[componentName] = false;
      });
    }
  }

  Future<void> _deleteComponent(String componentName) async {
    setState(() {
      _isLoading[componentName] = true;
    });

    try {
      String result;
      if (componentName == 'umu-launcher') {
        result = await api.deleteUmuLauncher();
      } else if (componentName == 'jadeite') {
        result = await api.deleteJadeite();
      } else {
        result = 'Unknown component';
      }

      if (result == 'ok') {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text('$componentName removed')),
          );
        }
        _loadComponents();
      } else {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text('Failed to remove: $result'),
              backgroundColor: Colors.red,
            ),
          );
        }
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Failed to remove: $e'),
            backgroundColor: Colors.red,
          ),
        );
      }
    } finally {
      setState(() {
        _isLoading[componentName] = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final wineRunners = _runners.where((r) => r.runnerType == 'wine').toList();
    final protonRunners =
        _runners.where((r) => r.runnerType == 'proton').toList();

    return SizedBox.expand(
      child: Container(
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
          child: SingleChildScrollView(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  'Components',
                  style: TextStyle(
                    color: ElysiaTheme.textPrimary,
                    fontSize: 32,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 8),
                const Text(
                  'Download and manage Wine/Proton runners and required components',
                  style: TextStyle(
                    color: ElysiaTheme.textSecondary,
                    fontSize: 14,
                  ),
                ),
                const SizedBox(height: 32),

                // Required Components Section
                _buildSection(
                  title: 'Required Components',
                  icon: Icons.extension,
                  children: _components
                      .map((component) => _ComponentListItem(
                            name: component.name,
                            displayName: component.displayName,
                            description: component.description,
                            version: component.version,
                            isInstalled: component.isInstalled,
                            isLoading: _isLoading[component.name] ?? false,
                            onInstall: () => _installComponent(component.name),
                            onDelete: () => _deleteComponent(component.name),
                          ))
                      .toList(),
                ),

                const SizedBox(height: 24),

                // Wine Runners Section
                _buildSection(
                  title: 'Wine Runners',
                  icon: Icons.wine_bar,
                  children: wineRunners
                      .map((runner) => _RunnerListItem(
                            name: runner.name,
                            displayName: runner.displayName,
                            version: runner.version,
                            runnerType: runner.runnerType,
                            isInstalled: runner.isInstalled,
                            isLoading: _isLoading[runner.name] ?? false,
                            onInstall: () => _installRunner(runner.name),
                            onDelete: () => _deleteRunner(runner.name),
                          ))
                      .toList(),
                ),

                const SizedBox(height: 24),

                // Proton Runners Section
                _buildSection(
                  title: 'Proton Runners',
                  icon: Icons.science,
                  children: protonRunners
                      .map((runner) => _RunnerListItem(
                            name: runner.name,
                            displayName: runner.displayName,
                            version: runner.version,
                            runnerType: runner.runnerType,
                            isInstalled: runner.isInstalled,
                            isLoading: _isLoading[runner.name] ?? false,
                            onInstall: () => _installRunner(runner.name),
                            onDelete: () => _deleteRunner(runner.name),
                          ))
                      .toList(),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildSection({
    required String title,
    required IconData icon,
    required List<Widget> children,
  }) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Icon(icon, color: ElysiaTheme.textSecondary, size: 20),
            const SizedBox(width: 8),
            Text(
              title,
              style: const TextStyle(
                color: ElysiaTheme.textSecondary,
                fontSize: 14,
                fontWeight: FontWeight.w500,
              ),
            ),
          ],
        ),
        const SizedBox(height: 12),
        Container(
          decoration: BoxDecoration(
            color: ElysiaTheme.surfaceColor.withValues(alpha: 0.5),
            borderRadius: BorderRadius.circular(ElysiaTheme.cardRadius),
            border: Border.all(color: ElysiaTheme.borderColor),
          ),
          child: Column(children: children),
        ),
      ],
    );
  }
}

// Component list item widget
class _ComponentListItem extends StatefulWidget {
  final String name;
  final String displayName;
  final String description;
  final String version;
  final bool isInstalled;
  final bool isLoading;
  final VoidCallback onInstall;
  final VoidCallback onDelete;

  const _ComponentListItem({
    required this.name,
    required this.displayName,
    required this.description,
    required this.version,
    required this.isInstalled,
    required this.isLoading,
    required this.onInstall,
    required this.onDelete,
  });

  @override
  State<_ComponentListItem> createState() => _ComponentListItemState();
}

class _ComponentListItemState extends State<_ComponentListItem> {
  bool _isHovering = false;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) => setState(() => _isHovering = true),
      onExit: (_) => setState(() => _isHovering = false),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 150),
        padding: const EdgeInsets.all(16),
        decoration: BoxDecoration(
          color: _isHovering
              ? ElysiaTheme.surfaceColor.withValues(alpha: 0.3)
              : Colors.transparent,
        ),
        child: Row(
          children: [
            Container(
              width: 48,
              height: 48,
              decoration: BoxDecoration(
                color: ElysiaTheme.cardColor,
                borderRadius: BorderRadius.circular(8),
              ),
              child: Icon(
                widget.name == 'umu-launcher'
                    ? Icons.rocket_launch
                    : Icons.shield,
                color: ElysiaTheme.accentColor,
                size: 24,
              ),
            ),
            const SizedBox(width: 16),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    widget.displayName,
                    style: const TextStyle(
                      color: ElysiaTheme.textPrimary,
                      fontSize: 16,
                      fontWeight: FontWeight.w500,
                    ),
                  ),
                  const SizedBox(height: 2),
                  Text(
                    widget.description,
                    style: const TextStyle(
                      color: ElysiaTheme.textSecondary,
                      fontSize: 12,
                    ),
                  ),
                  const SizedBox(height: 2),
                  Text(
                    'Version: ${widget.version}',
                    style: const TextStyle(
                      color: ElysiaTheme.textSecondary,
                      fontSize: 11,
                    ),
                  ),
                ],
              ),
            ),
            _buildStatusWidget(),
          ],
        ),
      ),
    );
  }

  Widget _buildStatusWidget() {
    if (widget.isLoading) {
      return const SizedBox(
        width: 24,
        height: 24,
        child: CircularProgressIndicator(
          strokeWidth: 2,
          color: ElysiaTheme.primaryColor,
        ),
      );
    }

    if (widget.isInstalled) {
      return Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
            decoration: BoxDecoration(
              color: Colors.green.withValues(alpha: 0.2),
              borderRadius: BorderRadius.circular(16),
              border: Border.all(color: Colors.green.withValues(alpha: 0.5)),
            ),
            child: const Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                Icon(Icons.check_circle, color: Colors.green, size: 16),
                SizedBox(width: 4),
                Text(
                  'Installed',
                  style: TextStyle(
                    color: Colors.green,
                    fontSize: 12,
                    fontWeight: FontWeight.w500,
                  ),
                ),
              ],
            ),
          ),
          const SizedBox(width: 8),
          IconButton(
            onPressed: widget.onDelete,
            icon: const Icon(Icons.delete_outline, color: Colors.red, size: 20),
            tooltip: 'Remove',
          ),
        ],
      );
    }

    return ElevatedButton.icon(
      onPressed: widget.onInstall,
      icon: const Icon(Icons.download, size: 18),
      label: const Text('Install'),
      style: ElevatedButton.styleFrom(
        backgroundColor: ElysiaTheme.primaryColor,
        foregroundColor: Colors.black,
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      ),
    );
  }
}

// Runner list item widget
class _RunnerListItem extends StatefulWidget {
  final String name;
  final String displayName;
  final String version;
  final String runnerType;
  final bool isInstalled;
  final bool isLoading;
  final VoidCallback onInstall;
  final VoidCallback onDelete;

  const _RunnerListItem({
    required this.name,
    required this.displayName,
    required this.version,
    required this.runnerType,
    required this.isInstalled,
    required this.isLoading,
    required this.onInstall,
    required this.onDelete,
  });

  @override
  State<_RunnerListItem> createState() => _RunnerListItemState();
}

class _RunnerListItemState extends State<_RunnerListItem> {
  bool _isHovering = false;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) => setState(() => _isHovering = true),
      onExit: (_) => setState(() => _isHovering = false),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 150),
        padding: const EdgeInsets.all(16),
        decoration: BoxDecoration(
          color: _isHovering
              ? ElysiaTheme.surfaceColor.withValues(alpha: 0.3)
              : Colors.transparent,
        ),
        child: Row(
          children: [
            Container(
              width: 48,
              height: 48,
              decoration: BoxDecoration(
                color: ElysiaTheme.cardColor,
                borderRadius: BorderRadius.circular(8),
              ),
              child: Icon(
                widget.runnerType == 'wine' ? Icons.wine_bar : Icons.science,
                color: ElysiaTheme.primaryColor,
                size: 24,
              ),
            ),
            const SizedBox(width: 16),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    widget.displayName,
                    style: const TextStyle(
                      color: ElysiaTheme.textPrimary,
                      fontSize: 16,
                      fontWeight: FontWeight.w500,
                    ),
                  ),
                  const SizedBox(height: 2),
                  Text(
                    'Version: ${widget.version}',
                    style: const TextStyle(
                      color: ElysiaTheme.textSecondary,
                      fontSize: 12,
                    ),
                  ),
                ],
              ),
            ),
            _buildStatusWidget(),
          ],
        ),
      ),
    );
  }

  Widget _buildStatusWidget() {
    if (widget.isLoading) {
      return const SizedBox(
        width: 24,
        height: 24,
        child: CircularProgressIndicator(
          strokeWidth: 2,
          color: ElysiaTheme.primaryColor,
        ),
      );
    }

    if (widget.isInstalled) {
      return Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
            decoration: BoxDecoration(
              color: Colors.green.withValues(alpha: 0.2),
              borderRadius: BorderRadius.circular(16),
              border: Border.all(color: Colors.green.withValues(alpha: 0.5)),
            ),
            child: const Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                Icon(Icons.check_circle, color: Colors.green, size: 16),
                SizedBox(width: 4),
                Text(
                  'Installed',
                  style: TextStyle(
                    color: Colors.green,
                    fontSize: 12,
                    fontWeight: FontWeight.w500,
                  ),
                ),
              ],
            ),
          ),
          const SizedBox(width: 8),
          IconButton(
            onPressed: widget.onDelete,
            icon: const Icon(Icons.delete_outline, color: Colors.red, size: 20),
            tooltip: 'Remove',
          ),
        ],
      );
    }

    return ElevatedButton.icon(
      onPressed: widget.onInstall,
      icon: const Icon(Icons.download, size: 18),
      label: const Text('Install'),
      style: ElevatedButton.styleFrom(
        backgroundColor: ElysiaTheme.primaryColor,
        foregroundColor: Colors.black,
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      ),
    );
  }
}
