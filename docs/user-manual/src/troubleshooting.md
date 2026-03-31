# Troubleshooting

If you encounter issues while using Seisly, check this guide for common solutions.

## Common Issues

### Application Fails to Start

- **Check GPU Support**: Seisly requires a GPU with Vulkan, Metal, or DirectX 12 support. Ensure your GPU drivers are up-to-date.
- **Log Files**: Check the `logs/` directory in your Seisly installation folder for any error messages or stack traces.

### Slow Rendering Performance

- **Update Drivers**: Older GPU drivers can cause performance bottlenecks.
- **Adjust Graphic Settings**: Go to **Settings → Graphics** and try lowering the **Rendering Quality** or **Texture Resolution**.
- **Large Datasets**: For very large seismic volumes, consider using a high-performance SSD for your Seisly projects.

### Problems Importing Data

- **SEG-Y Header Mappings**: Incorrectly defined inline/crossline header locations are a common cause of import failures. Double-check your SEG-Y file format specifications.
- **LAS Format Compatibility**: Ensure your LAS file follows the LAS 2.0 or 3.0 standard.

## Getting Support

If your issue is not listed here, please visit our community resources:

- **GitHub Issues**: [Report a bug](https://github.com/seisly/seisly/issues)
- **Discussions**: [Ask a question](https://github.com/seisly/seisly/discussions)
- **Discord**: [Join our Discord server](https://discord.gg/seisly)

## Debug Information

To help us diagnose your issue, please include the following information in your bug reports:

1. Seisly version (`seisly-app --version`)
2. Operating system and version
3. GPU model and driver version
4. Relevant log entries from `logs/seisly.log`
