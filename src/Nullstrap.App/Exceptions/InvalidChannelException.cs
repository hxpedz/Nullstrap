using System;
using System.Net;

namespace Nullstrap.Exceptions;

public class InvalidChannelException(HttpStatusCode? statusCode) : Exception
{
	public HttpStatusCode? StatusCode = statusCode;
}
