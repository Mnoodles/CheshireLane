from mitmproxy import http

def request(flow: http.HTTPFlow) -> None:
    if flow.request.host == "azurusapi.yo-star.com":
        flow.request.host = "101.6.41.183"     # Replace with your ip addr
        flow.request.port = 21080  # Default is 21080
        flow.request.scheme = "http"
        
    