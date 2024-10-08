# Main variables
SERVICES := appium android-simulator ios-simulator
TMP_DIR := tmp
IOS_SIMULATOR := iPhone 16

# Helper variables (not targets)
stop-services := $(addprefix stop-, $(SERVICES))
start-services := $(addprefix start-, $(SERVICES))
IOS_SIMULATOR_ID := $(shell xcrun simctl list --json devices available | jq '.devices[][] | select(.name == "$(IOS_SIMULATOR)") | .udid')

# Tell make that these targets aren't real files
.PHONY: install test help $(stop-services) $(start-services)

# When running just `make`, show our help message
default: help

install: ## Install dependencies
	npm install

stop: $(stop-services) ## Stop all services
	stty sane # Fix any terminal quirks

test: stop ## Run all tests
	$(MAKE) start-appium

	@echo "Run Android tests"
	$(MAKE) start-android-simulator
	node tests/android.test.cjs
	$(MAKE) stop-android-simulator

	@echo "Run iOS tests"
	$(MAKE) start-ios-simulator
	node tests/ios.test.cjs
	$(MAKE) stop-ios-simulator

	$(MAKE) stop-appium
	stty sane # Fix any terminal quirks
	@echo "\n\nTests completed"

start-appium: stop-appium ### Start the Appium test process
	# Run in the background and save the PID to a file
	npx appium --allow-insecure chromedriver_autodownload & echo $$! > $(TMP_DIR)/appium.pid

stop-appium: ### Stop the Appium test process
	[ -f $(TMP_DIR)/appium.pid ] && cat $(TMP_DIR)/appium.pid | xargs kill -2 || true
	rm -f $(TMP_DIR)/appium.pid

start-android-simulator: stop-android-simulator ### Start the Android simulator
	# Run in the background and save the PID to a file
	npm run tauri android dev & echo $$! > $(TMP_DIR)/android.pid
	sleep 45 # Wait enough time for the simulator to start

stop-android-simulator: ### Stop the Android simulator
	adb emu kill && sleep 15 || true
	[ -f $(TMP_DIR)/android.pid ] && cat $(TMP_DIR)/android.pid | xargs kill -2 || true
	rm -f $(TMP_DIR)/android.pid

start-ios-simulator: stop-ios-simulator ### Start the iOS simulator
	# Assuming we only have one simulator installed
	open -a Simulator && sleep 2
	xcrun simctl boot $(IOS_SIMULATOR_ID)
	# Run in the background and save the PID to a file
	npm run tauri ios dev "$(IOS_SIMULATOR)" & echo $$! > $(TMP_DIR)/ios.pid
	sleep 45 # Wait enough time for the simulator to start

stop-ios-simulator: ### Stop the iOS simulator
	xcrun simctl shutdown all
	osascript -e 'tell application "Simulator" to quit'
	[ -f $(TMP_DIR)/ios.pid ] && cat $(TMP_DIR)/ios.pid | xargs kill -2 || true
	rm -f $(TMP_DIR)/ios.pid

# https://gist.github.com/prwhite/8168133
# Add a double hash (##) after a target's description to make it show up in the help as a primary target.
# Add a triple hash (###) after a target's description to make it show up in the help as a secondary target.
help: ## Show this help.
	@echo '-- Primary targets --'
	@grep -hE '^[A-Za-z0-9_ \-]*?:.*\s##\s.*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-25s\033[0m %s\n", $$1, $$2}'
	@echo '-- Secondary targets --'
	@grep -hE '^[A-Za-z0-9_ \-]*?:.*\s###\s.*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?### "}; {printf "\033[36m%-25s\033[0m %s\n", $$1, $$2}'
