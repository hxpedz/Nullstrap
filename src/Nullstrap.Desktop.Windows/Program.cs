using Avalonia;
using Nullstrap.Desktop;
using Nullstrap.Platform.Windows;

namespace Nullstrap.Desktop.Windows;

internal static class Program
{
	[STAThread]
	public static void Main(string[] args)
	{
		DesktopRuntime.Configure(new WindowsPlatformHost(), args);
		BuildAvaloniaApp().StartWithClassicDesktopLifetime(args);
	}

	public static AppBuilder BuildAvaloniaApp()
	{
		return AppBuilder.Configure<DesktopApplication>().UsePlatformDetect();
	}
}
