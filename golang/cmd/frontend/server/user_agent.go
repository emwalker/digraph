package server

import (
	"github.com/avct/uasurfer"
)

type userAgent struct {
	*uasurfer.UserAgent
	uaString string
}

func newUserAgent(uaString string) userAgent {
	ua := uasurfer.Parse(uaString)
	return userAgent{ua, uaString}
}

func (ua userAgent) isChrome() bool {
	return ua.Browser.Name == uasurfer.BrowserChrome
}

func (ua userAgent) isPhone() bool {
	return ua.DeviceType == uasurfer.DevicePhone
}
