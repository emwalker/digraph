package redis

import (
	"context"
	"crypto/sha1"
	"fmt"
	"log"
	"time"

	"github.com/go-redis/redis/v8"
)

// A handle for a Redis connection
type RedisConnection struct {
	*redis.Client
}

type TestConnection struct{}

// A key for looking something up in Redis
type Key string

// A closure over code that will produce a result if the value cannot be found in Redis
type Thunk func() (*string, error)

type Connection interface {
	Fetch(context.Context, Key, Thunk) Result
	Ping(context.Context) *redis.StatusCmd
}

type Result struct {
	Payload *string
	Err     error
}

func (r Result) Success() bool {
	return r.Err == nil
}

type Options = redis.Options

// Return a new RedisConnection instance
func New(options *Options) Connection {
	client := redis.NewClient(options)
	return &RedisConnection{client}
}

// Create a Redis key
func NewKey(funcName string, identifier string) Key {
	h := sha1.New()
	h.Write([]byte(identifier))
	bs := h.Sum(nil)
	return Key(fmt.Sprintf("%s:%x", funcName, bs))
}

// Return whatever is found in Redis for `key`, and, if nothing is found, execute the thunk, save the
// value to Redis and return the result.
func (c *RedisConnection) Fetch(ctx context.Context, key Key, thunk Thunk) Result {
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

// Return a TestConnection instance, which satisfies the Connection interface and can be used in tests
func NewTestConnection(options *Options) Connection {
	return &TestConnection{}
}

// Allow the call to go to the database and return the result
func (c *TestConnection) Fetch(ctx context.Context, key Key, thunk Thunk) Result {
	str, err := thunk()
	if err != nil {
		return Result{nil, err}
	}
	return Result{str, nil}
}

// Pretend to ping the Redis instance
func (c *TestConnection) Ping(ctx context.Context) *redis.StatusCmd {
	return nil
}
