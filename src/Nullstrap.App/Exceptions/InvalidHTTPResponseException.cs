using System;

namespace Nullstrap.Exceptions;

internal sealed class InvalidHTTPResponseException(string message) : Exception(message)
{
}
