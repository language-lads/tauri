const { remote } = require("webdriverio");

const capabilities = {
  platformName: "iOS",
  "appium:automationName": "xcuitest",
  "appium:deviceName": "iPhone 16",
  "appium:appPackage": "com.lisalanguagelads.app",
  "appium:bundleId": "com.lisalanguagelads.app",
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
    //let packageName = await driver.getCurrentPackage();
    let contexts = await driver.execute("mobile: getContexts");
    let webview = contexts.find(
      (context) => context.bundleId === "com.lisalanguagelads.app",
    );
    await driver.switchContext(webview.id);
    await driver.pause(10000);
    console.log(await driver.getPageSource());
    await driver
      .$('//*[contains(text(), "Talk to me, boy")]')
      .waitForDisplayed();
    await driver
      .$('//button[contains(text(),"Hold to Record")]')
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
