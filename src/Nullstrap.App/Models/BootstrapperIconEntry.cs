using System.Windows.Media;
using Nullstrap.Enums;
using Nullstrap.Extensions;

namespace Nullstrap.Models;

public class BootstrapperIconEntry
{
	public BootstrapperIcon IconType { get; set; }

	public ImageSource ImageSource => IconEx.GetIconSource(IconType);
}
