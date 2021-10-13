package main

import (
	"log"
	"net"
)

func getOutBoundIP() net.IP {
	conn, err := net.Dial("udp", "8.8.8.8:8")

	if err != nil {
		log.Fatal("Cannot get IP", err)
	}

	defer conn.Close()

	localAddr := conn.LocalAddr().(*net.UDPAddr)

	return localAddr.IP
}
