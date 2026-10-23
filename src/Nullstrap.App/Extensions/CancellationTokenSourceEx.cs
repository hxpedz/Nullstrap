using System;
using System.Threading;

namespace Nullstrap.Extensions;

public static class CancellationTokenSourceEx
{
	private static readonly CancellationToken Cancelled = new(true);

	public static CancellationToken SafeToken(this CancellationTokenSource source)
	{
		try
		{
			return source.Token;
		}
		catch (ObjectDisposedException)
		{
			return Cancelled;
		}
	}
}
