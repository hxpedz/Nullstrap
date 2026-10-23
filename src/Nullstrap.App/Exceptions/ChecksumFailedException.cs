using System;

namespace Nullstrap.Exceptions;

internal sealed class ChecksumFailedException(string message) : Exception(message)
{
}
