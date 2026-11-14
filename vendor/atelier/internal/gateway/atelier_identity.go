package gateway

import (
	"net/http"
	"strings"
)

// AtelierIdentity accepts old local clients while emitting only canonical names.
// Request bodies, account ids and upstream URLs are deliberately left literal.
func AtelierIdentity(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		for _, prefix := range []string{"/v1/magpie/", "/_magpie/", "/mcp/magpie/"} {
			if strings.HasPrefix(r.URL.Path, prefix) {
				r.URL.Path = strings.Replace(r.URL.Path, "magpie", "atelier", 1)
				break
			}
		}
		for name, values := range r.Header {
			if strings.HasPrefix(name, "X-Magpie-") {
				canonical := strings.Replace(name, "X-Magpie-", "X-Atelier-", 1)
				if _, exists := r.Header[canonical]; !exists {
					r.Header[canonical] = values
				}
			}
		}
		for _, cookie := range r.Cookies() {
			if strings.HasPrefix(cookie.Name, "magpie_web_") {
				name := strings.Replace(cookie.Name, "magpie_web_", "atelier_web_", 1)
				if _, err := r.Cookie(name); err != nil {
					r.AddCookie(&http.Cookie{Name: name, Value: cookie.Value})
				}
			}
		}
		next.ServeHTTP(w, r)
	})
}
