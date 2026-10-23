﻿using System.Windows;
using Nullstrap.Platform;

namespace Nullstrap.UI;

public static class PlatformFeatureVisibility
{
	public static Visibility Overlay { get; } = Resolve(FeatureId.Overlay);

	public static Visibility GlobalInput { get; } = Resolve(FeatureId.GlobalInput);

	public static Visibility AudioSession { get; } = Resolve(FeatureId.AudioSession);

	public static Visibility Tray { get; } = Resolve(FeatureId.Tray);

	public static Visibility ResourceOptimization { get; } = Resolve(FeatureId.ResourceOptimization);

	public static Visibility TasxOptimization { get; } = Nullstrap.Utility.Platform.IsMacOS ? Visibility.Visible : ResourceOptimization;

	public static Visibility FrameGeneration { get; } = Resolve(FeatureId.FrameGeneration);

	public static Visibility VirtualController { get; } = Resolve(FeatureId.VirtualController);

	public static Visibility WindowsIntegration { get; } = Nullstrap.Utility.Platform.IsWindows ? Visibility.Visible : Visibility.Collapsed;

	public static Visibility NetworkMtu { get; } = Nullstrap.Utility.Platform.IsWindows ? Visibility.Visible : Visibility.Collapsed;

	public static Visibility DesktopBackdrop { get; } = Nullstrap.Utility.Platform.IsWindows || Nullstrap.Utility.Platform.IsMacOS ? Visibility.Visible : Visibility.Collapsed;

	public static Visibility LinuxIntegration { get; } = Nullstrap.Utility.Platform.IsLinux ? Visibility.Visible : Visibility.Collapsed;

	public static Visibility NotLinux { get; } = Nullstrap.Utility.Platform.IsLinux ? Visibility.Collapsed : Visibility.Visible;

	public static Visibility NotMacOS { get; } = Nullstrap.Utility.Platform.IsMacOS ? Visibility.Collapsed : Visibility.Visible;

	public static Visibility MacOSIntegration { get; } = Nullstrap.Utility.Platform.IsMacOS ? Visibility.Visible : Visibility.Collapsed;

	public static Visibility PortableImages { get; } = Nullstrap.Utility.Platform.UsesPortableUi ? Visibility.Visible : Visibility.Collapsed;

	public static bool IsSupported(FeatureId feature)
	{
		return Resolve(feature) == Visibility.Visible;
	}

	private static Visibility Resolve(FeatureId feature)
	{
		if (Nullstrap.Utility.Platform.IsWindows)
		{
			return Visibility.Visible;
		}

		try
		{
			IPlatformHost? host = Nullstrap.Utility.Platform.RuntimeHost;
			return Nullstrap.Core.PlatformFeatureGate.IsHidden(host?.Capabilities, feature)
				? Visibility.Collapsed
				: Visibility.Visible;
		}
		catch
		{
			return Visibility.Visible;
		}
	}
}
