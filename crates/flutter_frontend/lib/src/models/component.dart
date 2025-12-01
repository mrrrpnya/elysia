/// Component type enum for additional tools
enum ComponentType {
  umuLauncher,
  jadeite;

  String get directoryName {
    switch (this) {
      case ComponentType.umuLauncher:
        return 'umu';
      case ComponentType.jadeite:
        return 'tweaks/jadeite';
    }
  }

  String get displayName {
    switch (this) {
      case ComponentType.umuLauncher:
        return 'UMU Launcher';
      case ComponentType.jadeite:
        return 'Jadeite';
    }
  }
}

/// A component that can be downloaded (like umu-launcher, jadeite)
class AvailableComponent {
  final String name;
  final String displayName;
  final String description;
  final ComponentType componentType;
  final String version;
  final String downloadUrl;
  /// The file to check to verify installation
  final String checkFile;
  /// Archive type: 'tar', 'zip'
  final String archiveType;
  /// Whether to strip top-level directory when extracting
  final bool stripTopLevel;

  const AvailableComponent({
    required this.name,
    required this.displayName,
    required this.description,
    required this.componentType,
    required this.version,
    required this.downloadUrl,
    required this.checkFile,
    required this.archiveType,
    this.stripTopLevel = false,
  });
}

/// Status of a component download/installation
enum ComponentStatus {
  notInstalled,
  downloading,
  installing,
  installed,
  error,
}

/// Get available components list (umu-launcher, jadeite)
List<AvailableComponent> getAvailableComponents() {
  return const [
    AvailableComponent(
      name: 'umu-launcher',
      displayName: 'UMU Launcher',
      description: 'Required for running games with Proton',
      componentType: ComponentType.umuLauncher,
      version: '1.2.9',
      downloadUrl: 'https://github.com/Open-Wine-Components/umu-launcher/releases/download/1.2.9/umu-launcher-1.2.9-zipapp.tar',
      checkFile: 'umu-run',
      archiveType: 'tar',
      stripTopLevel: false,
    ),
    AvailableComponent(
      name: 'jadeite',
      displayName: 'Jadeite',
      description: 'Required for certain games (anti-cheat compatibility)',
      componentType: ComponentType.jadeite,
      version: 'v5.0.1',
      downloadUrl: 'https://codeberg.org/mkrsym1/jadeite/releases/download/v5.0.1/v5.0.1.zip',
      checkFile: 'jadeite.exe',
      archiveType: 'zip',
      stripTopLevel: false,
    ),
  ];
}
