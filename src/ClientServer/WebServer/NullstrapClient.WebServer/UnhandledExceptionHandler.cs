using System;
using NullstrapClient.Common;

namespace NullstrapClient.WebServer;

internal static class UnhandledExceptionHandler
{
	public static void Handle(object sender, UnhandledExceptionEventArgs args)
	{
		Exception ex = (Exception)args.ExceptionObject;
		try
		{
			Logger.Instance.Error("An unexpected error occured");
			Logger.Instance.Error(ex.ToString());
		}
		catch
		{
		}
	}
}
