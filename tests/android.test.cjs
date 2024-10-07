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
    // 1. To find the correct webview page for Android we need to switch to the webview first
    const webviewName = `WEBVIEW_${packageName}`;
    await driver.switchContext(webviewName);
    await driver.pause(15000);
		//let contexts = await driver.execute('mobile: getContexts')
		//let id = contexts[0].pages[0].id
		//console.log(contexts[0].pages[0].id)
		//await driver.switchToWindow(id);
		console.log(await driver.getPageSource());
		//const title = await driver.$('//*[@text="Talk to me, boy"]');
		const title = await driver.$("//button[contains(text(),'Hold to Record')]");
  } finally {
    await driver.pause(1000);
    await driver.deleteSession();
  }
}

runTest().catch((err) => {
  console.error(err);
  throw err;
});
