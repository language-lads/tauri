######################################################
#                                                    #
# Run `make help` to get a nice list of all commands #
#                                                    #
######################################################

# Main variables
SERVICES := appium android desktop
TMP_DIR := ./tmp
# We use UNAME_S to run different commands based on your OS
# E.g. don't run any iOS specific stuff unless we're on a Mac
UNAME_S := $(shell uname -s)

ifeq ($(UNAME_S),Darwin)
	# MacOS specific variables
  SERVICES += ios
  IOS_SIMULATOR := iPhone 16
  IOS_SIMULATOR_ID := $(shell xcrun simctl list --json devices available | jq '.devices[][] | select(.name == "$(IOS_SIMULATOR)") | .udid')
endif

# Helper variables (not targets)
stop-services := $(addprefix stop-, $(SERVICES))
start-services := $(addprefix start-, $(SERVICES))

# Tell make that these targets aren't real files
.PHONY: install test help info $(stop-services) $(start-services) info

# When running just `make`, show our help message
default: help

install: ## Install dependencies
	npm install

start: start-desktop ## Start the desktop app

stop: $(stop-services) ## Stop all services
	stty sane # Fix any terminal quirks

test: stop ## Run all tests
	$(MAKE) start-appium
	@echo "Run Android tests"
	$(MAKE) start-android
	node tests/android.test.cjs
	$(MAKE) stop-android
ifeq ($(UNAME_S),Darwin)
	@echo "Run iOS tests"
	$(MAKE) start-ios
	node tests/ios.test.cjs
	$(MAKE) stop-ios
endif
	$(MAKE) stop-appium
	stty sane # Fix any terminal quirks
	@echo "\n\nTests completed"

build: 
	npm run tauri build

info: ## Print out make variables
	@echo "SERVICES: $(SERVICES)"
	@echo "TMP_DIR: $(TMP_DIR)"
	@echo "UNAME_S: $(UNAME_S)"
	@echo "MAKEFILE_LIST: $(MAKEFILE_LIST)"
ifeq ($(UNAME_S),Darwin)
	@echo "IOS_SIMULATOR: $(IOS_SIMULATOR)"
	@echo "IOS_SIMULATOR_ID: $(IOS_SIMULATOR_ID)"
endif

start-desktop: stop-desktop ### Run the desktop app
	npm run tauri dev & 
	sleep 20 # Wait for the desktop app to start

stop-desktop: ### Stop the desktop app
	pkill -SIGINT -U $$(whoami) -f "npm run tauri dev" || true
	pkill -SIGINT -U $$(whoami) -f "src-tauri/target/debug" || true

start-android: stop-android ### Run the app on Android
	# Run in the background and save the PID to a file
	npm run tauri android dev &
	sleep 60 # Wait for the simulator to start and the app to be installed

stop-android: ### Stop the Android app and simulator
	adb emu kill && sleep 30 || true
	pkill -SIGINT -U $$(whoami) -f "npm run tauri android dev" || true

ifeq ($(UNAME_S),Darwin)
start-ios: stop-ios ### Run the app on iOS
	# Assuming we only have one simulator installed
	open -a Simulator && sleep 5
	xcrun simctl boot $(IOS_SIMULATOR_ID)
	# Run in the background and save the PID to a file
	npm run tauri ios dev "$(IOS_SIMULATOR)" &
	sleep 60 # Wait enough time for the simulator to start and the app to be installed

stop-ios: ### Stop the iOS app and simulator
	xcrun simctl shutdown all
	osascript -e 'tell application "Simulator" to quit'
	pkill -SIGINT -U $$(whoami) -f "npm run tauri ios dev" || true
endif

start-appium: stop-appium ### Start the Appium test process
	# Run in the background and save the PID to a file
	npx appium --allow-insecure chromedriver_autodownload &

stop-appium: ### Stop the Appium test process
	pkill -SIGINT -U $$(whoami) -f "npm exec appium" || true

# https://gist.github.com/prwhite/8168133
# Add a double hash (##) after a target's description to make it show up in the help as a primary target.
# Add a triple hash (###) after a target's description to make it show up in the help as a secondary target.
help: ## Show this help.
	@echo "\n-- Primary targets"
	@grep -hE '^[A-Za-z0-9_ \-]*?:.*\s##\s.*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
	@echo "\n-- Secondary targets"
	@grep -hE '^[A-Za-z0-9_ \-]*?:.*\s###\s.*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?### "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
