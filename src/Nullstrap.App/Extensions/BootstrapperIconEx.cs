using System;
using System.Collections.Generic;
using System.Drawing;
using Nullstrap.Enums;
using Nullstrap.Properties;

namespace Nullstrap.Extensions;

internal static class BootstrapperIconEx
{
	public static IReadOnlyCollection<BootstrapperIcon> Selections { get; } = new BootstrapperIcon[9]
	{
		BootstrapperIcon.IconNullstrap,
		BootstrapperIcon.Icon2022,
		BootstrapperIcon.Icon2019,
		BootstrapperIcon.Icon2017,
		BootstrapperIcon.IconLate2015,
		BootstrapperIcon.IconEarly2015,
		BootstrapperIcon.Icon2011,
		BootstrapperIcon.Icon2008,
		BootstrapperIcon.IconCustom
	};

	public static Icon GetIcon(this BootstrapperIcon icon)
	{
		switch (icon)
		{
		case BootstrapperIcon.IconCustom:
		{
			Icon? icon2 = null;
			string bootstrapperIconCustomLocation = App.Settings.Prop.BootstrapperIconCustomLocation;
			if (string.IsNullOrEmpty(bootstrapperIconCustomLocation))
			{
				App.Logger.WriteLine("BootstrapperIconEx::GetIcon", "Warning: custom icon is not set.");
			}
			else
			{
				try
				{
					icon2 = new Icon(bootstrapperIconCustomLocation);
				}
				catch (Exception ex)
				{
					App.Logger.WriteLine("BootstrapperIconEx::GetIcon", "Failed to load custom icon!");
					App.Logger.WriteException("BootstrapperIconEx::GetIcon", ex);
				}
			}
			return icon2 ?? Nullstrap.Properties.Resources.IconNullstrap;
		}
		case BootstrapperIcon.IconNullstrap:
			return Nullstrap.Utility.Branding.CreateDrawingIcon() ?? Nullstrap.Properties.Resources.IconNullstrap;
		case BootstrapperIcon.Icon2008:
			return Nullstrap.Properties.Resources.Icon2008;
		case BootstrapperIcon.Icon2011:
			return Nullstrap.Properties.Resources.Icon2011;
		case BootstrapperIcon.IconEarly2015:
			return Nullstrap.Properties.Resources.IconEarly2015;
		case BootstrapperIcon.IconLate2015:
			return Nullstrap.Properties.Resources.IconLate2015;
		case BootstrapperIcon.Icon2017:
			return Nullstrap.Properties.Resources.Icon2017;
		case BootstrapperIcon.Icon2019:
			return Nullstrap.Properties.Resources.Icon2019;
		case BootstrapperIcon.Icon2022:
			return Nullstrap.Properties.Resources.Icon2022;
		default:
			return Nullstrap.Properties.Resources.IconNullstrap;
		}
	}
}
