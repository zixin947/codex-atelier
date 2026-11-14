package gui

// Atelier owns the listener and gateway lifecycle. All page assets and API
// handlers below are upstream's original implementations.
import (
	"github.com/zixin947/codex-atelier/router/internal/gateway"
	"net"
	"net/http"
	"net/url"
	"time"
)

func AtelierUI(gw *gateway.Server, key string) (string, func(), error) {
	listener, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		return "", nil, err
	}
	_, port, _ := net.SplitHostPort(listener.Addr().String())
	server := &http.Server{Handler: webServed(port, key, 0, noFramedAPI(atelierChatGPT(Handler(webHost{quit: func() {}}, gw)))), ReadHeaderTimeout: 30 * time.Second}
	go server.Serve(listener)
	return "http://" + listener.Addr().String() + "/?view=agents&k=" + url.QueryEscape(key), func() { server.Close() }, nil
}
