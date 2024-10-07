import SwiftRs
import Tauri
import UIKit
import WebKit
import AVFoundation
import os.log

class PingArgs: Decodable {
  let value: String?
}

class ExamplePlugin: Plugin {
  @objc public func ping(_ invoke: Invoke) throws {
    os_log("Hellow boall")

    do {
        let audioSession = AVAudioSession.sharedInstance()
        try audioSession.setCategory(.playAndRecord, mode: .default)
        try audioSession.setActive(true)
    } catch {
        print("Failed to set up audio session: \(error)")
    }
    let args = try invoke.parseArgs(PingArgs.self)
    invoke.resolve(["value": args.value ?? ""])
  }
}

@_cdecl("init_plugin_microphone")
func initPlugin() -> Plugin {
  return ExamplePlugin()
}
