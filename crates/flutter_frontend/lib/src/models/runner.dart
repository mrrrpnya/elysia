/// Runner type enum
enum RunnerType {
  wine,
  proton;

  String get directoryName {
    switch (this) {
      case RunnerType.wine:
        return 'wine';
      case RunnerType.proton:
        return 'proton';
    }
  }
}

/// A runner that can be downloaded and installed
class AvailableRunner {
  final String name;
  final String displayName;
  final RunnerType runnerType;
  final String version;
  final String downloadUrl;
  final String folderName;

  const AvailableRunner({
    required this.name,
    required this.displayName,
    required this.runnerType,
    required this.version,
    required this.downloadUrl,
    required this.folderName,
  });
}

/// Status of a runner download/installation
enum RunnerStatus {
  notInstalled,
  downloading,
  installing,
  installed,
  error,
}

/// An installed runner
class InstalledRunner {
  final String name;
  final String displayName;
  final RunnerType runnerType;
  final String version;
  final String path;

  const InstalledRunner({
    required this.name,
    required this.displayName,
    required this.runnerType,
    required this.version,
    required this.path,
  });
}

/// Get available runners list
List<AvailableRunner> getAvailableRunners() {
  return const [
    AvailableRunner(
      name: 'wine-tkg-aagl',
      displayName: 'Wine-TKG AAGL',
      runnerType: RunnerType.wine,
      version: 'v10.15-7',
      downloadUrl: 'https://github.com/NelloKudo/Wine-Builds/releases/download/wine-tkg-aagl-v10.15-7/wine-tkg-aagl-v10.15-7-x86_64.tar.xz',
      folderName: 'wine-tkg-aagl-v10.15-7',
    ),
    AvailableRunner(
      name: 'dwproton',
      displayName: 'DW Proton',
      runnerType: RunnerType.proton,
      version: '10.0-9',
      downloadUrl: 'https://dawn.wine/dawn-winery/dwproton/releases/download/dwproton-10.0-9/dwproton-10.0-9.tar.xz',
      folderName: 'GE-Proton',
    ),
  ];
}
