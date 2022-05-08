package redis

import (
	"context"
	"crypto/sha1"
	"fmt"
	"log"
	"time"

	"github.com/go-redis/redis/v8"
)

// RealConnection handles a Redis connection
type RealConnection struct {
	*redis.Client
}

// TestConnection is a fake connection used for testing
type TestConnection struct{}

// Key is used for looking something up in Redis
type Key string

// Thunk is a closure over code that will produce a result if the value cannot be found in Redis
type Thunk func() (*string, error)

// Connection provides an interface for the connection used for fetching Redis keys
type Connection interface {
	Fetch(context.Context, Key, Thunk) Result
	Ping(context.Context) *redis.StatusCmd
}

// Result is the return value for a Redis lookup
type Result struct {
	Payload *string
	Err     error
}

// Success returns true if the lookup was a success
func (r Result) Success() bool {
	return r.Err == nil
}

// Options is an alias for go-redis options
type Options = redis.Options

// New returns a new RealConnection instance
func New(options *Options) Connection {
	client := redis.NewClient(options)
	return &RealConnection{client}
}

// NewKey creates a Redis key
func NewKey(funcName string, identifier string) Key {
	h := sha1.New()
	h.Write([]byte(identifier))
	bs := h.Sum(nil)
	return Key(fmt.Sprintf("%s:%x", funcName, bs))
}

// Fetch returns whatever is found in Redis for `key`, and, if nothing is found, execute the thunk, save the
// value to Redis and return the result.
func (c *RealConnection) Fetch(ctx context.Context, key Key, thunk Thunk) Result {
	cachedString, err := c.Get(ctx, string(key)).Result()
	if err != nil && err != redis.Nil {
		return Result{nil, err}
	}

	if err == nil {
		log.Printf("Found key in redis: %s", key)
		return Result{&cachedString, nil}
	}

	str, err := thunk()
	if err != nil {
		return Result{nil, err}
	}

	log.Printf("Setting %s in redis", key)
	_, err = c.SetNX(ctx, string(key), *str, 10*time.Minute).Result()
	if err != nil {
		log.Printf("Problem setting %s in redis: %s", key, err)
		return Result{nil, err}
	}

	return Result{str, nil}
}

// NewTestConnection returns a TestConnection instance, which satisfies the Connection interface and can be
// used in tests
func NewTestConnection(options *Options) Connection {
	return &TestConnection{}
}

// Fetch allows the call to go to the database and return the result
func (c *TestConnection) Fetch(ctx context.Context, key Key, thunk Thunk) Result {
	str, err := thunk()
	if err != nil {
		return Result{nil, err}
	}
	return Result{str, nil}
}

// Ping pretends to ping the Redis instance
func (c *TestConnection) Ping(ctx context.Context) *redis.StatusCmd {
	return nil
}
