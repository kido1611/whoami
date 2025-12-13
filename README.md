# ifconfig.me

Self-host ifconfig.me, Detect your IP address.

## How To

The usage is simple, just access `http://localhost:8080` using browser and the result will be in HTML format.

Also can be accessed using HTTP Client (like `curl`, `httpie`) to get in plain text.

Add header `Accept: application/json` to get result as JSON format.

## Available Routes

| path        | description                        | plain text | JSON | HTML |
| ----------- | ---------------------------------- | ---------- | ---- | ---- |
| /           | Default page, show all information | ✅         | ✅   | ✅   |
| /ip         | Get IP Address only                | ✅         | ✅   |      |
| /user-agent | Get User-Agent only                | ✅         | ✅   |      |

## Configuration

All configurations can be changed from Environment Variables.

| Name               | Default Value | Description                                                                                                             |
| ------------------ | ------------- | ----------------------------------------------------------------------------------------------------------------------- |
| IFCONFIG_PORT      | 8080          | Change app port                                                                                                         |
| IFCONFIG_IP_SOURCE | ConnectInfo   | IP Detection source. Used this when run behind reverse proxy. See IP source section (below) to see all available values |

### IP Source

This is all the available options for IP Source:

```
/// IP from the `CF-Connecting-IP` header
CfConnectingIp

/// IP from the `CloudFront-Viewer-Address` header
CloudFrontViewerAddress

/// IP from the `Fly-Client-IP` header
FlyClientIp

/// Rightmost IP from the `X-Forwarded-For` header
RightmostXForwardedFor

/// IP from the `True-Client-IP` header
TrueClientIp

/// IP from the `X-Real-Ip` header
XRealIp

/// Default, used this when the app does not run behind reverse proxy
ConnectInfo
```

## TODO

- [ ] add api list section, which contains api route list and command to run using `curl`, `httpie`, or etc.
- [ ] add detail information of the ip, like location (country, latitude, longitude), timezone, etc.
