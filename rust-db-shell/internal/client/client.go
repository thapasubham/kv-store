package client

import (
	"fmt"
	"net"
	"strings"
	"time"
)

type Client struct {
	addr string
	conn net.Conn
}

const defaultTimeout = 5 * time.Second

func NewClient(addr string) *Client {
	return &Client{addr: addr}
}

func (c *Client) Connect() error {
	conn, err := net.DialTimeout("tcp", c.addr, defaultTimeout)
	if err != nil {
		return fmt.Errorf("connect: %w", err)
	}
	c.conn = conn
	return nil
}

func (c *Client) Ping() error {
	conn, err := net.DialTimeout("tcp", c.addr, defaultTimeout)
	if err != nil {
		return err
	}
	return conn.Close()
}

func (c *Client) Command(cmd string) (string, error) {
	if c.conn == nil {
		return "", fmt.Errorf("no active connection available")
	}

	line := strings.TrimSpace(cmd) + "\n"
	if _, err := c.conn.Write([]byte(line)); err != nil {
		return "", fmt.Errorf("failed to stream data: %w", err)
	}

	buf := make([]byte, 4096)
	n, err := c.conn.Read(buf)
	if err != nil {
		return "", fmt.Errorf("failed to read data stream: %w", err)
	}

	return string(buf[:n]), nil
}
