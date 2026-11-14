package gateway

import (
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestAtelierIdentity(t *testing.T) {
	for _, path := range []string{"/v1/magpie/events", "/_magpie/plugin/a", "/mcp/magpie/search", "/v1/atelier/events"} {
		t.Run(path, func(t *testing.T) {
			r := httptest.NewRequest("GET", path, nil)
			r.Header.Set("X-Magpie-Client", "legacy")
			r.AddCookie(&http.Cookie{Name: "magpie_web_9000", Value: "secret"})
			handler := AtelierIdentity(http.HandlerFunc(func(w http.ResponseWriter, got *http.Request) {
				if got.URL.Path == path && path != "/v1/atelier/events" {
					t.Fatal(got.URL.Path)
				}
				if got.Header.Get("X-Atelier-Client") != "legacy" {
					t.Fatal(got.Header)
				}
				cookie, err := got.Cookie("atelier_web_9000")
				if err != nil || cookie.Value != "secret" {
					t.Fatal(cookie, err)
				}
			}))
			handler.ServeHTTP(httptest.NewRecorder(), r)
		})
	}
}
