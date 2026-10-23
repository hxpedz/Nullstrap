using Avalonia;
using Nullstrap.Desktop;
using Nullstrap.Platform.MacOS;

namespace Nullstrap.Desktop.MacOS;

internal static class Program
{
	[STAThread]
	public static void Main(string[] args)
	{
		DesktopRuntime.Configure(new MacOSPlatformHost(), args);
		BuildAvaloniaApp().StartWithClassicDesktopLifetime(args);
	}

	public static AppBuilder BuildAvaloniaApp()
	{
		return AppBuilder.Configure<DesktopApplication>().UsePlatformDetect();
	}
}
