package gateway

import (
	"context"
	"github.com/zixin947/codex-atelier/router/internal/provider"
	"net/http"
)

// AtelierHandler keeps the original listener's access checks in the embedded host.
func (s *Server) AtelierHandler() http.Handler {
	return lanGuard(s.Handler())
}

// AtelierShutdown exposes the core's existing drain sequence to its embedding
// host, which owns the HTTP listener instead of ListenAndServe's background jobs.
func (s *Server) AtelierShutdown(ctx context.Context, server *http.Server) error {
	s.endStreams()
	sockets := make(chan struct{})
	go func() { defer close(sockets); s.sockets.drain(ctx) }()
	err := server.Shutdown(ctx)
	<-sockets
	s.waitSightSaves(ctx)
	s.subscription.abortAll()
	provider.FlushServed()
	return err
}
