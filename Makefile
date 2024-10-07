TMP_DIR := tmp
default: help

#.PHONY: $(addprefix stop-,$(BACKGROUND_SERVICES))
.PHONY: stop-android stop-appium start-appium start-android-simulator test help

install: ## Install dependencies
	npm install
	npx appium driver install uiautomator2

stop: stop-android-simulator stop-appium ## Stop all background services

start-appium: stop-appium ## Start the Appium test process
	# Run in the background and save the PID to a file
	npx appium --allow-insecure chromedriver_autodownload & echo $$! > tmp/appium.pid

stop-appium: ## Stop the Appium test process
	[ -f tmp/appium.pid ] && cat tmp/appium.pid | xargs kill -2 || true
	rm -f tmp/appium.pid

start-android-simulator: stop-android-simulator ## Start the Android simulator
	# Run in the background and save the PID to a file
	npm run tauri android dev & echo $$! > tmp/android-simulator.pid
	sleep 45 # Wait enough time for the simulator to start

stop-android-simulator: ## Stop the Android simulator
	adb emu kill || true
	[ -f tmp/android-simulator.pid ] && cat tmp/android-simulator.pid | xargs kill -2 || true
	rm -f tmp/android-simulator.pid
	sleep 15 # Wait enough time for the simulator to stop


## This is a bit of 'make' wizardry that defines a 'stop' target for each of our BACKGROUND_SERVICES
#stop-background-services := $(addprefix stop-, $(BACKGROUND_SERVICES))
#${stop-background-services}: stop-%:
#  [ -f $(TMP_DIR)/$*.pid ] && cat $(TMP_DIR)/$*.pid | xargs kill -9 || true
#  rm -f $(TMP_DIR)/$*.pid
#stop: ${stop-background-services} ## Stop all background services

test: ## Run all tests
	$(MAKE) start-appium
	$(MAKE) start-android-simulator
	node tests/settings.test.cjs
	$(MAKE) stop-appium
	$(MAKE) stop-android-simulator
	@echo "\n\nTests completed"


# https://gist.github.com/prwhite/8168133
# Add a double hash (##) after a target's description to make it show up in the help.
help:	## Show this help.
	@grep -hE '^[A-Za-z0-9_ \-]*?:.*##.*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-25s\033[0m %s\n", $$1, $$2}'


#stop-android-simulator:
#  [ -f tmp/android.pid ] && cat tmp/android.pid | xargs kill || true
#  rm -f tmp/android.pid

#stop-ios-simulator:
#  [ -f tmp/ios.pid ] && cat tmp/ios.pid | xargs kill || true
#  rm -f tmp/ios.pid

#stop-appium:
#  [ -f tmp/appium.pid ] && cat tmp/appium.pid | xargs kill || true
#  rm -f tmp/appium.pid
