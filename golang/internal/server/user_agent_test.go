package server

import (
	"testing"

	"github.com/avct/uasurfer"
)

func TestCombinations(t *testing.T) {
	testData := []struct {
		name        string
		uaString    string
		browserName uasurfer.BrowserName
		deviceType  uasurfer.DeviceType
		isChrome    bool
		isPhone     bool
	}{
		{
			name:        "It detects iOS Chrome",
			uaString:    "Mozilla/5.0 (iPhone; CPU iPhone OS 12_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/72.0.3626.101 Mobile/15E148 Safari/605.1",
			browserName: uasurfer.BrowserChrome,
			deviceType:  uasurfer.DevicePhone,
			isChrome:    true,
			isPhone:     true,
		},
		{
			name:        "It detects iOS Safari",
			uaString:    "Mozilla/5.0 (iPhone; CPU iPhone OS 12_1_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/12.0 Mobile/15E148 Safari/604.1",
			browserName: uasurfer.BrowserSafari,
			deviceType:  uasurfer.DevicePhone,
			isChrome:    false,
			isPhone:     true,
		},
	}

	for _, td := range testData {
		t.Run(td.name, func(t *testing.T) {
			ua := newUserAgent(td.uaString)

			if td.browserName != ua.Browser.Name {
				t.Fatalf("Expected browser %#v, got %#v", td.browserName, ua.Browser.Name)
			}

			if td.isChrome != ua.isChrome() {
				t.Fatalf("Expected isChrome %t, got %t", td.isChrome, ua.isChrome())
			}

			if td.deviceType != ua.DeviceType {
				t.Fatalf("Expected device type %#v, got %#v", td.deviceType, ua.DeviceType)
			}

			if td.isPhone != ua.isPhone() {
				t.Fatalf("Expected isPhone %t, got %t", td.isPhone, ua.isPhone())
			}
		})
	}
}
