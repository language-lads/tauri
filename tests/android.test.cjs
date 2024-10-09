const { remote } = require("webdriverio");

const capabilities = {
  platformName: "Android",
  "appium:automationName": "UiAutomator2",
  "appium:deviceName": "Android",
  "appium:appPackage": "com.lisalanguagelads.app",
  "appium:appActivity": ".MainActivity",
};

const wdOpts = {
  hostname: process.env.APPIUM_HOST || "localhost",
  port: parseInt(process.env.APPIUM_PORT, 10) || 4723,
  logLevel: "info",
  capabilities,
};

async function runTest() {
  const driver = await remote(wdOpts);
  try {
    // https://github.com/webdriverio/appium-boilerplate/blob/main/tests/helpers/WebView.ts
    let packageName = await driver.getCurrentPackage();
    const webviewName = `WEBVIEW_${packageName}`;
    await driver.switchContext(webviewName);
    await driver
      .$('//*[contains(text(), "Talk to me, boy")]')
			.waitForDisplayed({ timeout: 20000 });
    await driver
      .$('//button[contains(text(),"Start conversation")]')
      .waitForDisplayed();
    await driver
      .$('//*[contains(text(), "I DONT EXIST")]')
			.waitForDisplayed({ reverse: true });
  } finally {
    await driver.pause(1000);
    await driver.deleteSession();
  }
}

runTest().catch((err) => {
  console.error(err);
  throw err;
});
